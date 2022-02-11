#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]

use crate::{bnst_compare, case_data, dic, Dpnd_matrix, match_matrix, path_matrix, tools};
use crate::ctools::stdout;


#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;
/* #define FOR_GLOSS 訳挿入用 */
#[no_mangle]
pub unsafe extern "C" fn write_head(mut fp: *mut tools::FILE) {
    let mut fpr: *mut tools::FILE = 0 as *mut tools::FILE;
    let mut c: libc::c_int = 0;
    fpr =
        fopen(b"/home/kuro/work/nl/E/knpe/src/head.dat\x00" as *const u8 as
                  *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    loop  {
        c = fgetc(fpr);
        if !(c != -(1 as libc::c_int)) { break ; }
        fputc(c, fp);
    }
    fclose(fpr);
}
#[no_mangle]
pub unsafe extern "C" fn write_tail(mut fp: *mut tools::FILE) {
    let mut fpr: *mut tools::FILE = 0 as *mut tools::FILE;
    let mut c: libc::c_int = 0;
    fpr =
        fopen(b"/home/kuro/work/nl/E/knpe/src/tail.dat\x00" as *const u8 as
                  *const libc::c_char,
              b"r\x00" as *const u8 as *const libc::c_char);
    loop  {
        c = fgetc(fpr);
        if !(c != -(1 as libc::c_int)) { break ; }
        fputc(c, fp);
    }
    fclose(fpr);
}
#[no_mangle]
pub unsafe extern "C" fn write_text10(mut fp: *mut tools::FILE,
                                      mut cp: *mut libc::c_char,
                                      mut x: libc::c_int,
                                      mut y: libc::c_int) {
    fprintf(fp, b"Begin %%I Text\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"%%I cfg Black\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"0 0 0 SetCFg\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp,
            b"%%I f *-times-medium-r-*-100-*\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(fp,
            b"/Times-Roman 10 SetF\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(fp, b"%%I t\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp,
            b"[ 1 0 0 1 %d %d ] concat\n\x00" as *const u8 as
                *const libc::c_char, x, y - 3 as libc::c_int);
    fprintf(fp, b"%%I\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"[\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"(%s)\n\x00" as *const u8 as *const libc::c_char, cp);
    fprintf(fp, b"] WhiteBg Text\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"End\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn write_text14(mut fp: *mut tools::FILE,
                                      mut cp: *mut libc::c_char,
                                      mut x: libc::c_int,
                                      mut y: libc::c_int) {
    fprintf(fp, b"Begin %%I Text\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"%%I cfg Black\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"0 0 0 SetCFg\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp,
            b"%%I f *-times-medium-r-*-140-*\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(fp,
            b"/Times-Roman 14 SetF\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(fp, b"%%I t\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp,
            b"[ 1 0 0 1 %d %d ] concat\n\x00" as *const u8 as
                *const libc::c_char, x, y - 3 as libc::c_int);
    fprintf(fp, b"%%I\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"[\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"(%s)\n\x00" as *const u8 as *const libc::c_char, cp);
    fprintf(fp, b"] WhiteBg Text\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"End\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn write_kanji(mut fp: *mut tools::FILE,
                                     mut cp: *mut libc::c_char,
                                     mut x: libc::c_int, mut y: libc::c_int) {
    fprintf(fp, b"Begin %%I KText\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"%%I cfg Black\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"0 0 0 SetCFg\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"%%I k k14\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp,
            b"/Ryumin-Light-H 14 SetF\n\x00" as *const u8 as
                *const libc::c_char);
    fprintf(fp, b"%%I t\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp,
            b"[ 1 0 0 1 %d %d ] concat\n\x00" as *const u8 as
                *const libc::c_char, x, y);
    fprintf(fp, b"%%I\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"[\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"<\x00" as *const u8 as *const libc::c_char);
    while *cp != 0 {
        fprintf(fp, b"%02x\x00" as *const u8 as *const libc::c_char,
                *cp as libc::c_int + 128 as libc::c_int);
        cp = cp.offset(1)
    }
    fprintf(fp, b">\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"] WhiteBg Text\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"End\n\x00" as *const u8 as *const libc::c_char);
    fprintf(fp, b"\n\x00" as *const u8 as *const libc::c_char);
}
static mut tmp: [libc::c_char; 64] = [0; 64];
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_matrix2ps(mut sp: *mut tools::SENTENCE_DATA,
                                         mut jlen: libc::c_int,
                                         mut type_0: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0; /* para_key の表示用 */
    let mut j: libc::c_int = 0;
    let mut comma_p: libc::c_int = 0;
    let mut para_char: libc::c_int = 0 as libc::c_int;
    let mut point_B: [libc::c_char; 10] = [0; 10];
    let mut ptr: *mut tools::PARA_DATA = 0 as *mut tools::PARA_DATA;
    let mut b_ptr: *mut tools::BNST_DATA = 0 as *mut tools::BNST_DATA;
    /* パスのマーク付け */
    i = 0 as libc::c_int;
    while i < (*sp).Bnst_num {
        j = 0 as libc::c_int;
        while j < (*sp).Bnst_num {
            (*path_matrix.as_mut_ptr().offset(i as isize))[j as usize] =
                0 as libc::c_int;
            j += 1
        }
        i += 1
    }
    if type_0 == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*sp).Para_num {
            ptr = &mut *(*sp).para_data.offset(i as isize) as *mut tools::PARA_DATA;
            j = (*ptr).key_pos + 1 as libc::c_int;
            while j <= (*ptr).jend_pos {
                (*path_matrix.as_mut_ptr().offset((*ptr).max_path[(j -
                                                                       (*ptr).key_pos
                                                                       -
                                                                       1 as
                                                                           libc::c_int)
                                                                      as
                                                                      usize]
                                                      as isize))[j as usize] =
                    if (*path_matrix.as_mut_ptr().offset((*ptr).max_path[(j -
                                                                              (*ptr).key_pos
                                                                              -
                                                                              1
                                                                                  as
                                                                                  libc::c_int)
                                                                             as
                                                                             usize]
                                                             as
                                                             isize))[j as
                                                                         usize]
                           != 0 {
                        -(1 as libc::c_int)
                    } else { ('a' as i32) + i };
                j += 1
            }
            i += 1
        }
    }
    /* ＰＳの出力 */
    write_head(stdout);
    i = 0 as libc::c_int;
    b_ptr = (*sp).bnst_data;
    while i < (*sp).Bnst_num {
        if (*b_ptr).mrph_num == 1 as libc::c_int {
            strcpy(tmp.as_mut_ptr(), (*(*b_ptr).mrph_ptr).Goi2.as_mut_ptr());
            comma_p = 0 as libc::c_int
        } else {
            strcpy(tmp.as_mut_ptr(), (*(*b_ptr).mrph_ptr).Goi2.as_mut_ptr());
            j = 1 as libc::c_int;
            while j < (*b_ptr).mrph_num - 1 as libc::c_int {
                strcat(tmp.as_mut_ptr(),
                       (*(*b_ptr).mrph_ptr.offset(j as
                                                      isize)).Goi2.as_mut_ptr());
                j += 1
            }
            if strcmp(Class[(*(*b_ptr).mrph_ptr.offset((*b_ptr).mrph_num as
                                                           isize).offset(-(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize))).Hinshi
                                as usize][0 as libc::c_int as usize].id as
                          *const libc::c_char,
                      b"\xe7\x89\xb9\xe6\xae\x8a\x00" as *const u8 as
                          *const libc::c_char) == 0 &&
                   strcmp(Class[(*(*b_ptr).mrph_ptr.offset((*b_ptr).mrph_num
                                                               as
                                                               isize).offset(-(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize))).Hinshi
                                    as
                                    usize][(*(*b_ptr).mrph_ptr.offset((*b_ptr).mrph_num
                                                                          as
                                                                          isize).offset(-(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize))).Bunrui
                                               as usize].id as
                              *const libc::c_char,
                          b"\xe8\xaa\xad\xe7\x82\xb9\x00" as *const u8 as
                              *const libc::c_char) == 0 {
                comma_p = (0 as libc::c_int == 0) as libc::c_int
            } else {
                strcat(tmp.as_mut_ptr(),
                       (*(*b_ptr).mrph_ptr.offset((*b_ptr).mrph_num as
                                                      isize).offset(-(1 as
                                                                          libc::c_int
                                                                          as
                                                                          isize))).Goi2.as_mut_ptr());
                comma_p = 0 as libc::c_int
            }
        }
        if (*(*sp).bnst_data.offset(i as isize)).para_key_type != 0 {
            sprintf(point_B.as_mut_ptr(),
                    b"%c>\x00" as *const u8 as *const libc::c_char,
                    'a' as i32 + para_char);
            para_char += 1;
            if comma_p != 0 {
                write_text14(stdout, point_B.as_mut_ptr(),
                             ((197 as libc::c_int + i * 20 as libc::c_int +
                                   15 as libc::c_int) as
                                  libc::c_ulong).wrapping_sub(strlen(tmp.as_mut_ptr()).wrapping_mul(7
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong)).wrapping_sub(14
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_ulong).wrapping_sub(7
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_int
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_ulong)
                                 as libc::c_int,
                             757 as libc::c_int - i * 20 as libc::c_int);
            } else {
                write_text14(stdout, point_B.as_mut_ptr(),
                             ((197 as libc::c_int + i * 20 as libc::c_int +
                                   15 as libc::c_int) as
                                  libc::c_ulong).wrapping_sub(strlen(tmp.as_mut_ptr()).wrapping_mul(7
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_ulong)).wrapping_sub(14
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_ulong)
                                 as libc::c_int,
                             757 as libc::c_int - i * 20 as libc::c_int);
            }
        }
        if comma_p != 0 {
            write_kanji(stdout, tmp.as_mut_ptr(),
                        ((197 as libc::c_int + i * 20 as libc::c_int +
                              15 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen(tmp.as_mut_ptr()).wrapping_mul(7
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)).wrapping_sub(7
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_ulong)
                            as libc::c_int,
                        757 as libc::c_int - i * 20 as libc::c_int);
            write_text14(stdout,
                         b",\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char,
                         197 as libc::c_int + i * 20 as libc::c_int +
                             15 as libc::c_int - 7 as libc::c_int,
                         757 as libc::c_int - i * 20 as libc::c_int);
        } else {
            write_kanji(stdout, tmp.as_mut_ptr(),
                        ((197 as libc::c_int + i * 20 as libc::c_int +
                              15 as libc::c_int) as
                             libc::c_ulong).wrapping_sub(strlen(tmp.as_mut_ptr()).wrapping_mul(7
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong))
                            as libc::c_int,
                        757 as libc::c_int - i * 20 as libc::c_int);
        }
        j = i + 1 as libc::c_int;
        while j < (*sp).Bnst_num {
            if type_0 == 0 as libc::c_int {
                sprintf(point_B.as_mut_ptr(),
                        b"%2d\x00" as *const u8 as *const libc::c_char,
                        (*match_matrix.as_mut_ptr().offset(i as
                                                               isize))[j as
                                                                           usize]);
            } else if type_0 == 1 as libc::c_int {
                sprintf(point_B.as_mut_ptr(),
                        b"%2d\x00" as *const u8 as *const libc::c_char,
                        (*Dpnd_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]);
            }
            match (*path_matrix.as_mut_ptr().offset(i as isize))[j as usize] {
                0 => {
                    point_B[2 as libc::c_int as usize] =
                        ' ' as i32 as libc::c_char
                }
                -1 => {
                    point_B[2 as libc::c_int as usize] =
                        '*' as i32 as libc::c_char
                }
                _ => {
                    point_B[2 as libc::c_int as usize] =
                        (*path_matrix.as_mut_ptr().offset(i as
                                                              isize))[j as
                                                                          usize]
                            as libc::c_char
                }
            }
            point_B[3 as libc::c_int as usize] =
                '\u{0}' as i32 as libc::c_char;
            write_text14(stdout, point_B.as_mut_ptr(),
                         197 as libc::c_int + j * 20 as libc::c_int,
                         757 as libc::c_int - i * 20 as libc::c_int);
            j += 1
        }
        i += 1;
        b_ptr = b_ptr.offset(1)
    }
    if type_0 == 0 as libc::c_int {
        sprintf(point_B.as_mut_ptr(),
                b"\\(%d\x00" as *const u8 as *const libc::c_char, jlen);
        write_text14(stdout, point_B.as_mut_ptr(),
                     ((197 as libc::c_int +
                           ((*sp).Bnst_num - 1 as libc::c_int) *
                               20 as libc::c_int + 15 as libc::c_int) as
                          libc::c_ulong).wrapping_sub(strlen(point_B.as_mut_ptr()).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong).wrapping_mul(7
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_ulong)).wrapping_sub(35
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_ulong)
                         as libc::c_int,
                     757 as libc::c_int - (*sp).Bnst_num * 20 as libc::c_int);
        write_kanji(stdout,
                    b"\xe6\x96\x87\xe5\xad\x97\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    197 as libc::c_int +
                        ((*sp).Bnst_num - 1 as libc::c_int) *
                            20 as libc::c_int + 15 as libc::c_int -
                        35 as libc::c_int,
                    757 as libc::c_int - (*sp).Bnst_num * 20 as libc::c_int);
        write_text14(stdout,
                     b"\\)\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                     197 as libc::c_int +
                         ((*sp).Bnst_num - 1 as libc::c_int) *
                             20 as libc::c_int + 15 as libc::c_int -
                         7 as libc::c_int,
                     757 as libc::c_int - (*sp).Bnst_num * 20 as libc::c_int);
    }
    write_tail(stdout);
}
static mut X_pos: libc::c_int = 0;
static mut Y_pos: libc::c_int = 0;
static mut Wid: libc::c_int = 0;
static mut Hig: libc::c_int = 0;
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_bnst2ps(mut ptr: *mut tools::BNST_DATA)
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    if !ptr.is_null() {
        if (*ptr).para_top_p as libc::c_int ==
               (0 as libc::c_int == 0) as libc::c_int {
            write_text10(stdout,
                         b"PARA\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, X_pos, Y_pos);
            X_pos += Wid * 4 as libc::c_int
        } else {
            strcpy(tmp.as_mut_ptr(), (*(*ptr).mrph_ptr).Goi2.as_mut_ptr());
            i = 1 as libc::c_int;
            while i < (*ptr).mrph_num {
                strcat(tmp.as_mut_ptr(),
                       (*(*ptr).mrph_ptr.offset(i as
                                                    isize)).Goi2.as_mut_ptr());
                i += 1
            }
            write_kanji(stdout, tmp.as_mut_ptr(), X_pos, Y_pos);
            X_pos =
                (X_pos as
                     libc::c_ulong).wrapping_add((Wid as
                                                      libc::c_ulong).wrapping_mul(strlen(tmp.as_mut_ptr())))
                    as libc::c_int as libc::c_int
        }
        if (*ptr).para_type as libc::c_int == 1 as libc::c_int {
            write_text10(stdout,
                         b"<P>\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, X_pos, Y_pos);
            X_pos += Wid * 3 as libc::c_int
        } else if (*ptr).para_type as libc::c_int == 2 as libc::c_int {
            write_text10(stdout,
                         b"<I>\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, X_pos, Y_pos);
            X_pos += Wid * 3 as libc::c_int
        }
        if (*ptr).to_para_p as libc::c_int ==
               (0 as libc::c_int == 0) as libc::c_int {
            write_text10(stdout,
                         b"\\(D\\)\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char, X_pos, Y_pos);
            X_pos += Wid * 3 as libc::c_int
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn calc_tree_width2ps(mut ptr: *mut tools::BNST_DATA,
                                            mut depth2: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    if (*ptr).para_top_p as libc::c_int ==
           (0 as libc::c_int == 0) as libc::c_int {
        (*ptr).space = 4 as libc::c_int
    } else { (*ptr).space = (*ptr).length }
    if (*ptr).para_type as libc::c_int == 1 as libc::c_int ||
           (*ptr).para_type as libc::c_int == 2 as libc::c_int {
        (*ptr).space += 1 as libc::c_int
    }
    if (*ptr).to_para_p as libc::c_int ==
           (0 as libc::c_int == 0) as libc::c_int {
        (*ptr).space += 3 as libc::c_int
    }
    (*ptr).space += (depth2 - 1 as libc::c_int) * 8 as libc::c_int;
    if !(*ptr).child[0 as libc::c_int as usize].is_null() {
        i = 0 as libc::c_int;
        while !(*ptr).child[i as usize].is_null() {
            calc_tree_width2ps((*ptr).child[i as usize],
                               depth2 + 1 as libc::c_int);
            i += 1
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn show_link2ps(mut depth: libc::c_int,
                                      mut ans_flag: *mut libc::c_char,
                                      mut para_flag: libc::c_int,
                                      mut x_pos: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    if depth != 1 as libc::c_int {
        if para_flag == 1 as libc::c_int || para_flag == 2 as libc::c_int {
            write_kanji(stdout,
                        b"\xe2\x94\x80\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, X_pos,
                        Y_pos);
            X_pos += Wid * 2 as libc::c_int
        } else {
            write_kanji(stdout,
                        b"\xe2\x94\x80\xe2\x94\x80\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, X_pos,
                        Y_pos);
            X_pos += Wid * 4 as libc::c_int
        }
        if *ans_flag.offset((depth - 1 as libc::c_int) as isize) as
               libc::c_int == '1' as i32 {
            write_kanji(stdout,
                        b"\xe2\x94\xa4\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, X_pos,
                        Y_pos);
            X_pos += Wid * 2 as libc::c_int
        } else {
            write_kanji(stdout,
                        b"\xe2\x94\x90\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, X_pos,
                        Y_pos);
            X_pos += Wid * 2 as libc::c_int
        }
        X_pos += Wid * 2 as libc::c_int;
        i = depth - 1 as libc::c_int;
        while i > 1 as libc::c_int {
            X_pos += Wid * 4 as libc::c_int;
            if *ans_flag.offset((i - 1 as libc::c_int) as isize) as
                   libc::c_int == '1' as i32 {
                write_kanji(stdout,
                            b"\xe2\x94\x82\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            X_pos, Y_pos);
                X_pos += Wid * 2 as libc::c_int
            } else { X_pos += Wid * 2 as libc::c_int }
            X_pos += Wid * 2 as libc::c_int;
            i -= 1
        }
        /* 訳挿入用 */
        Y_pos -= Hig;
        X_pos = 497 as libc::c_int
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn show_self2ps(mut ptr: *mut tools::BNST_DATA,
                                      mut depth: libc::c_int,
                                      mut ans_flag_p: *mut libc::c_char,
                                      mut flag: libc::c_int) 
 /*==================================================================*/
 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ans_flag: [libc::c_char; 200] = [0; 200];
    if !ans_flag_p.is_null() {
        strncpy(ans_flag.as_mut_ptr(), ans_flag_p,
                200 as libc::c_int as libc::c_ulong);
    } else {
        ans_flag[0 as libc::c_int as usize] = '0' as i32 as libc::c_char
        /* 最初に呼ばれるとき */
    }
    if !(*ptr).child[0 as libc::c_int as usize].is_null() {
        i = 0 as libc::c_int;
        while !(*ptr).child[i as usize].is_null() { i += 1 }
        /* 最後の子は ans_flag を 0 に */
        ans_flag[depth as usize] = '0' as i32 as libc::c_char;
        show_self2ps((*ptr).child[(i - 1 as libc::c_int) as usize],
                     depth + 1 as libc::c_int, ans_flag.as_mut_ptr(),
                     0 as libc::c_int);
        if i > 1 as libc::c_int {
            /* 他の子は ans_flag を 1 に */
            ans_flag[depth as usize] = '1' as i32 as libc::c_char;
            j = i - 2 as libc::c_int;
            while j > 0 as libc::c_int {
                show_self2ps((*ptr).child[j as usize],
                             depth + 1 as libc::c_int, ans_flag.as_mut_ptr(),
                             0 as libc::c_int);
                j -= 1
            }
            /* flag: 1: ─PARA 2: -<P>PARA */
            if (*ptr).para_top_p as libc::c_int ==
                   (0 as libc::c_int == 0) as libc::c_int &&
                   (*ptr).para_type as libc::c_int == 0 as libc::c_int {
                show_self2ps((*ptr).child[0 as libc::c_int as usize],
                             depth + 1 as libc::c_int, ans_flag.as_mut_ptr(),
                             1 as libc::c_int);
            } else if (*ptr).para_top_p as libc::c_int ==
                          (0 as libc::c_int == 0) as libc::c_int &&
                          (*ptr).para_type as libc::c_int != 0 as libc::c_int
             {
                show_self2ps((*ptr).child[0 as libc::c_int as usize],
                             depth + 1 as libc::c_int, ans_flag.as_mut_ptr(),
                             2 as libc::c_int);
            } else {
                show_self2ps((*ptr).child[0 as libc::c_int as usize],
                             depth + 1 as libc::c_int, ans_flag.as_mut_ptr(),
                             0 as libc::c_int);
            }
        }
    }
    if (*ptr).para_top_p as libc::c_int !=
           (0 as libc::c_int == 0) as libc::c_int {
        X_pos -= (*ptr).space * 7 as libc::c_int
    }
    print_bnst2ps(ptr);
    if flag == 0 as libc::c_int {
        show_link2ps(depth, ans_flag.as_mut_ptr(),
                     (*ptr).para_type as libc::c_int, X_pos);
    } else if flag == 1 as libc::c_int {
        write_kanji(stdout,
                    b"\xe2\x94\x80\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, X_pos, Y_pos);
        X_pos += Wid * 2 as libc::c_int
    } else if flag == 2 as libc::c_int {
        write_text10(stdout,
                     b"-\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char, X_pos, Y_pos);
        X_pos += Wid
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn print_kakari2ps(mut sp: *mut tools::SENTENCE_DATA)
 /*==================================================================*/
 {
    /* 依存構造木の表示 */
    calc_tree_width2ps((*sp).bnst_data.offset((*sp).Bnst_num as
                                                  isize).offset(-(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)),
                       1 as libc::c_int);
    X_pos = 497 as libc::c_int;
    Y_pos = 757 as libc::c_int;
    Wid = 7 as libc::c_int;
    Hig = 14 as libc::c_int;
    /* ＰＳの出力 */
    write_head(stdout);
    show_self2ps((*sp).bnst_data.offset((*sp).Bnst_num as
                                            isize).offset(-(1 as libc::c_int
                                                                as isize)),
                 1 as libc::c_int, 0 as *mut libc::c_char, 0 as libc::c_int);
    write_tail(stdout);
}
/*====================================================================
                               END
====================================================================*/
