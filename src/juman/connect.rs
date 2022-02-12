#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::juman::consts::{OpenError, OtherError};
use crate::juman::ctools::{CurPath, error, fclose, fprintf, fputs, fscanf, getpath, JumanPath, my_alloc, pathfopen, print_current_time, ProgName, strcmp, strcpy, strrchr};
use crate::juman::types::{FILE, MRPH, RENSETU_PAIR};

static mut TBL_NUM: libc::c_int = 0;
/* 連接表のサイズ */
static mut I_NUM: libc::c_int = 0;
/* 連接行列の行   */
static mut J_NUM: libc::c_int = 0;
/* 連接行列の列   */
#[no_mangle]
pub static mut rensetu_tbl: *mut RENSETU_PAIR = 0 as *const RENSETU_PAIR as *mut RENSETU_PAIR;
#[no_mangle]
pub static mut rensetu_mtr: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
/*
------------------------------------------------------------------------------
        rensetu table
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn connect_table(mut fp_out: *mut FILE) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tablefile_path: [libc::c_char; 4096] = [0; 4096];
    let mut prog_basename: *mut libc::c_char = 0 as *mut libc::c_char;
    getpath(CurPath.as_mut_ptr(), JumanPath.as_mut_ptr());
    /* program basename (juman, makeint, ...) */
    if !ProgName.is_null() {
        prog_basename = strrchr(ProgName, '/' as i32);
        if !prog_basename.is_null() {
            prog_basename = prog_basename.offset(1)
        } else { prog_basename = ProgName }
    }
    loop  {
        fp =
            pathfopen(b"jumandic.tab\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, tablefile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        fp =
            pathfopen(b"jumandic.tab\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, CurPath.as_mut_ptr(),
                      tablefile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        if !prog_basename.is_null() &&
               strcmp(prog_basename,
                      b"juman\x00" as *const u8 as *const libc::c_char) != 0
               &&
               {
                   fp =
                       pathfopen(b"jumandic.tab\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 b"r\x00" as *const u8 as *const libc::c_char
                                     as *mut libc::c_char,
                                 b"../dic/\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 tablefile_path.as_mut_ptr());
                   !fp.is_null()
               } {
            break ;
        }
        fp =
            pathfopen(b"jumandic.tab\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, JumanPath.as_mut_ptr(),
                      tablefile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        error(OpenError as libc::c_int,
              b"can\'t open\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char,
              b"jumandic.tab\x00" as *const u8 as *const libc::c_char,
              b".\x00" as *const u8 as *const libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    if !fp_out.is_null() {
        print_current_time(fp_out);
        fprintf(fp_out,
                b"%s parsing... \x00" as *const u8 as *const libc::c_char,
                tablefile_path.as_mut_ptr());
    }
    read_table(fp);
    if !fp_out.is_null() {
        fputs(b"done.\n\n\x00" as *const u8 as *const libc::c_char, fp_out);
    }
    fclose(fp);
}
#[no_mangle]
pub unsafe extern "C" fn read_table(mut fp: *mut FILE) {
    let mut i: libc::c_int = 0;
    let mut tmp_char: [libc::c_uchar; 129] = [0; 129];
    fscanf(fp, b"%d\n\x00" as *const u8 as *const libc::c_char,
           &mut TBL_NUM as *mut libc::c_int);
    rensetu_tbl =
        my_alloc((::std::mem::size_of::<RENSETU_PAIR>() as
                      libc::c_ulong).wrapping_mul(TBL_NUM as libc::c_ulong) as
                     libc::c_int) as *mut RENSETU_PAIR;
    i = 0 as libc::c_int;
    while i < TBL_NUM {
        fscanf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
               &mut (*rensetu_tbl.offset(i as isize)).i_pos as
                   *mut libc::c_int);
        fscanf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
               &mut (*rensetu_tbl.offset(i as isize)).j_pos as
                   *mut libc::c_int);
        fscanf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
               &mut (*rensetu_tbl.offset(i as isize)).hinsi as
                   *mut libc::c_int);
        fscanf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
               &mut (*rensetu_tbl.offset(i as isize)).bunrui as
                   *mut libc::c_int);
        fscanf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
               &mut (*rensetu_tbl.offset(i as isize)).type_0 as
                   *mut libc::c_int);
        fscanf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
               &mut (*rensetu_tbl.offset(i as isize)).form as
                   *mut libc::c_int);
        fscanf(fp, b"%s\n\x00" as *const u8 as *const libc::c_char,
               tmp_char.as_mut_ptr());
        if tmp_char[0 as libc::c_int as usize] as libc::c_int == '*' as i32 {
            let ref mut fresh0 = (*rensetu_tbl.offset(i as isize)).goi;
            *fresh0 = 0 as *mut libc::c_uchar
        } else {
            let ref mut fresh1 = (*rensetu_tbl.offset(i as isize)).goi;
            *fresh1 =
                my_alloc((::std::mem::size_of::<libc::c_uchar>() as
                              libc::c_ulong).wrapping_mul(129 as libc::c_int
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int) as *mut libc::c_uchar;
            strcpy((*rensetu_tbl.offset(i as isize)).goi as *mut libc::c_char,
                   tmp_char.as_mut_ptr() as *const libc::c_char);
            /* 連接テーブルは基本形 */
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn check_table(mut mrph_p: *mut MRPH) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < TBL_NUM {
        if (*rensetu_tbl.offset(i as isize)).hinsi ==
               (*mrph_p).hinsi as libc::c_int &&
               (*rensetu_tbl.offset(i as isize)).bunrui ==
                   (*mrph_p).bunrui as libc::c_int &&
               (*rensetu_tbl.offset(i as isize)).type_0 ==
                   (*mrph_p).katuyou1 as libc::c_int &&
               ((*rensetu_tbl.offset(i as isize)).goi.is_null() ||
                    strcmp((*rensetu_tbl.offset(i as isize)).goi as
                               *const libc::c_char,
                           (*mrph_p).midasi2.as_mut_ptr() as
                               *const libc::c_char) == 0 as libc::c_int) {
            (*mrph_p).con_tbl = i;
            return
        }
        i += 1
    }
    error(OtherError as libc::c_int,
          b"No morpheme in table !!\x00" as *const u8 as *const libc::c_char
              as *mut libc::c_char, -(1 as libc::c_int) as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn check_table_for_rengo(mut mrph_p: *mut MRPH) {
    /* 連語用：形態素との違いは，語が一致すること，指定なければ -1 */
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < TBL_NUM {
        if (*rensetu_tbl.offset(i as isize)).hinsi == 127 as libc::c_int &&
               (*rensetu_tbl.offset(i as isize)).type_0 ==
                   (*mrph_p).katuyou1 as libc::c_int &&
               strcmp((*rensetu_tbl.offset(i as isize)).goi as
                          *const libc::c_char,
                      (*mrph_p).midasi2.as_mut_ptr() as *const libc::c_char)
                   == 0 as libc::c_int {
            (*mrph_p).con_tbl = i;
            if (*mrph_p).katuyou1 != 0 {
                (*mrph_p).con_tbl +=
                    (*mrph_p).katuyou2 as libc::c_int - 1 as libc::c_int
            }
            return
        }
        i += 1
    }
    (*mrph_p).con_tbl = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn check_table_for_undef(mut hinsi: libc::c_int,
                                               mut bunrui: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < TBL_NUM {
        if (*rensetu_tbl.offset(i as isize)).hinsi == hinsi &&
               (*rensetu_tbl.offset(i as isize)).bunrui == bunrui &&
               (*rensetu_tbl.offset(i as isize)).goi.is_null() {
            return i
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
/*
------------------------------------------------------------------------------
        rensetu matrix
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn connect_matrix(mut fp_out: *mut FILE) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut matrixfile_path: [libc::c_char; 4096] = [0; 4096];
    getpath(CurPath.as_mut_ptr(), JumanPath.as_mut_ptr());
    loop  {
        fp =
            pathfopen(b"jumandic.mat\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, matrixfile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        fp =
            pathfopen(b"jumandic.mat\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, CurPath.as_mut_ptr(),
                      matrixfile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        fp =
            pathfopen(b"jumandic.mat\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"r\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, JumanPath.as_mut_ptr(),
                      matrixfile_path.as_mut_ptr());
        if !fp.is_null() { break ; }
        error(OpenError as libc::c_int,
              b"can\'t open\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char,
              b"jumandic.mat\x00" as *const u8 as *const libc::c_char,
              b".\x00" as *const u8 as *const libc::c_char,
              -(1 as libc::c_int) as *mut libc::c_char);
    }
    if !fp_out.is_null() {
        print_current_time(fp_out);
        fprintf(fp_out,
                b"%s parsing... \x00" as *const u8 as *const libc::c_char,
                matrixfile_path.as_mut_ptr());
    }
    read_matrix(fp);
    if !fp_out.is_null() {
        fputs(b"done.\n\n\x00" as *const u8 as *const libc::c_char, fp_out);
    }
    fclose(fp);
}
#[no_mangle]
pub unsafe extern "C" fn read_matrix(mut fp: *mut FILE) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    fscanf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
           &mut I_NUM as *mut libc::c_int);
    fscanf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
           &mut J_NUM as *mut libc::c_int);
    rensetu_mtr =
        my_alloc((::std::mem::size_of::<libc::c_uchar>() as
                      libc::c_ulong).wrapping_mul(I_NUM as
                                                      libc::c_ulong).wrapping_mul(J_NUM
                                                                                      as
                                                                                      libc::c_ulong)
                     as libc::c_int) as *mut libc::c_uchar;
    i = 0 as libc::c_int;
    while i < I_NUM {
        j = 0 as libc::c_int;
        while j < J_NUM {
            if fscanf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
                      &mut num as *mut libc::c_int) == -(1 as libc::c_int) {
                error(OtherError as libc::c_int,
                      b"No entry in matrix !!\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      -(1 as libc::c_int) as *mut libc::c_char);
            }
            *rensetu_mtr.offset((i * J_NUM + j) as isize) =
                num as libc::c_char as libc::c_uchar;
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn check_matrix(mut postcon: libc::c_int,
                                      mut precon: libc::c_int)
 -> libc::c_int {
    if postcon == -(1 as libc::c_int) || precon == -(1 as libc::c_int) {
        return 10 as libc::c_int
    }
    return *rensetu_mtr.offset(((*rensetu_tbl.offset(postcon as isize)).i_pos
                                    * J_NUM +
                                    (*rensetu_tbl.offset(precon as
                                                             isize)).j_pos) as
                                   isize) as libc::c_int;
}
/* その連語に特有の左連接規則が記述されているか */
#[no_mangle]
pub unsafe extern "C" fn check_matrix_left(mut precon: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if precon == -(1 as libc::c_int) { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while i < I_NUM {
        if *rensetu_mtr.offset((i * J_NUM +
                                    (*rensetu_tbl.offset(precon as
                                                             isize)).j_pos) as
                                   isize) as libc::c_int != 0 {
            return (0 as libc::c_int == 0) as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/* その連語に特有の右連接規則が記述されているか */
#[no_mangle]
pub unsafe extern "C" fn check_matrix_right(mut postcon: libc::c_int)
 -> libc::c_int {
    let mut j: libc::c_int = 0;
    if postcon == -(1 as libc::c_int) { return 0 as libc::c_int }
    j = 0 as libc::c_int;
    while j < J_NUM {
        if *rensetu_mtr.offset(((*rensetu_tbl.offset(postcon as isize)).i_pos
                                    * J_NUM + j) as isize) as libc::c_int != 0
           {
            return (0 as libc::c_int == 0) as libc::c_int
        }
        j += 1
    }
    return 0 as libc::c_int;
}
