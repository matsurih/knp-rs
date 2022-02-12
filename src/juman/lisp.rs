#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::juman::consts::{DicError, OtherError, ProgramError};
use crate::juman::ctools::{error, feof, fgetc, fprintf, fputc, getc, LineNo, LineNoForError, my_alloc, my_exit, stderr, strcmp, strcpy, strlen, ungetc};
use crate::juman::structs::C2RustUnnamed;
use crate::juman::types::{BIN, CELL, CELLTABLE, FILE};

#[no_mangle]
pub static mut Buffer: [libc::c_uchar; 1025] = [0; 1025];
#[no_mangle]
pub static mut _TmpCell: CELL =
    CELL{tag: 0,
         value: C2RustUnnamed{
             cons: BIN{
                 car: 0 as *const libc::c_void as *mut libc::c_void,
                 cdr: 0 as *const libc::c_void as *mut libc::c_void,
             },
         },
    };
#[no_mangle]
pub static mut TmpCell: *mut CELL = unsafe { &_TmpCell as *const CELL as *mut CELL };
#[no_mangle]
pub static mut CellTbl: *mut CELLTABLE = 0 as *const CELLTABLE as *mut CELLTABLE;
#[no_mangle]
pub static mut CellTbl_save: CELLTABLE = CELLTABLE{
    pre: 0 as *const libc::c_void as *mut libc::c_void,
    next: 0 as *const libc::c_void as *mut libc::c_void,
    max: 0,
    n: 0,
    cell: 0 as *const CELL as *mut CELL,
};
static mut my_getc: Option<unsafe extern "C" fn(_: *mut FILE) -> libc::c_int> =
    unsafe {
        Some(fgetc as unsafe extern "C" fn(_: *mut FILE) -> libc::c_int)
    };
static mut my_ungetc:
       Option<unsafe extern "C" fn(_: libc::c_int, _: *mut FILE)
                  -> libc::c_int> =
    unsafe {
        Some(ungetc as
                 unsafe extern "C" fn(_: libc::c_int, _: *mut FILE)
                     -> libc::c_int)
    };
static mut is_bol: libc::c_int = 1 as libc::c_int;
static mut c_stacked: libc::c_int = -(1 as libc::c_int);
unsafe extern "C" fn cha_getc(mut fp: *mut FILE) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if c_stacked != -(1 as libc::c_int) {
        c = c_stacked;
        c_stacked = -(1 as libc::c_int)
    } else { c = getc(fp) }
    if c == '\r' as i32 { c = getc(fp) }
    if c == 0xb as libc::c_int && is_bol != 0 {
        c = getc(fp);
        if c == '\r' as i32 { c = getc(fp) }
        if c == '\n' as i32 { c = -(1 as libc::c_int) }
    }
    if c == '\n' as i32 || c == -(1 as libc::c_int) {
        is_bol = 1 as libc::c_int
    } else { is_bol = 0 as libc::c_int }
    return c;
}
unsafe extern "C" fn cha_ungetc(mut c: libc::c_int, mut fp: *mut FILE)
 -> libc::c_int {
    c_stacked = c;
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn set_cha_getc() {
    my_getc =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: *mut FILE)
                                           ->
                                               libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                                   *mut FILE)
                                                                                              ->
                                                                                                  libc::c_int,
                                                                                          unsafe extern "C" fn()
                                                                                              ->
                                                                                                  libc::c_int>(cha_getc)));
    my_ungetc =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> libc::c_int>,
                                Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _: *mut FILE)
                                           ->
                                               libc::c_int>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                                   libc::c_int,
                                                                                                               _:
                                                                                                                   *mut FILE)
                                                                                              ->
                                                                                                  libc::c_int,
                                                                                          unsafe extern "C" fn()
                                                                                              ->
                                                                                                  libc::c_int>(cha_ungetc)));
}
#[no_mangle]
pub unsafe extern "C" fn unset_cha_getc() {
    my_getc =
        Some(fgetc as unsafe extern "C" fn(_: *mut FILE) -> libc::c_int);
    my_ungetc =
        Some(ungetc as
                 unsafe extern "C" fn(_: libc::c_int, _: *mut FILE)
                     -> libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn error_in_lisp() {
    fprintf(stderr,
            b"\nparse error between line %d and %d.\n\x00" as *const u8 as
                *const libc::c_char, LineNoForError, LineNo);
    my_exit(DicError as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn error_in_program() {
    fprintf(stderr,
            b"\n\"ifnextchar\" returns an unexpected code.\n\x00" as *const u8
                as *const libc::c_char);
    my_exit(ProgramError as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn s_feof(mut fp: *mut FILE) -> libc::c_int {
    let mut code: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if s_feof_comment(fp) == -(1 as libc::c_int) {
        code = (0 as libc::c_int == 0) as libc::c_int
    } else {
        c = my_getc.expect("non-null function pointer")(fp);
        if c == -(1 as libc::c_int) {
            code = (0 as libc::c_int == 0) as libc::c_int
        } else if c as libc::c_uchar as libc::c_int == '\n' as i32 {
            LineNo += 1;
            code = s_feof(fp)
        } else if c as libc::c_uchar as libc::c_int == ' ' as i32 ||
                      c as libc::c_uchar as libc::c_int == '\t' as i32 {
            code = s_feof(fp)
        } else {
            my_ungetc.expect("non-null function pointer")(c, fp);
            code = 0 as libc::c_int
        }
    }
    return code;
}
#[no_mangle]
pub unsafe extern "C" fn s_feof_comment(mut fp: *mut FILE) -> libc::c_int {
    let mut n: libc::c_int = 0;
    // let mut Buffer_0: [libc::c_uchar; 1025] = [0; 1025];
    n = ifnextchar(fp, ';' as i32);
    if n == (0 as libc::c_int == 0) as libc::c_int {
        while my_getc.expect("non-null function pointer")(fp) != '\n' as i32 && feof(fp) == 0 {}
        LineNo += 1;
        return s_feof_comment(fp)
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn make_cell() -> *mut CELL {
    return lisp_alloc(::std::mem::size_of::<CELL>() as libc::c_ulong as libc::c_int) as *mut CELL;
}
#[no_mangle]
pub unsafe extern "C" fn tmp_atom(mut atom: *mut libc::c_uchar) -> *mut CELL {
    (*TmpCell).tag = 1 as libc::c_int;
    (*TmpCell).value.atom = atom;
    return TmpCell;
}
#[no_mangle]
pub unsafe extern "C" fn cons(mut car_0: *mut libc::c_void,
                              mut cdr_0: *mut libc::c_void) -> *mut CELL {
    let mut cell: *mut CELL = 0 as *mut CELL;
    cell = make_cell();
    (*cell).tag = 0 as libc::c_int;
    (*cell).value.cons.car = car_0;
    (*cell).value.cons.cdr = cdr_0;
    return cell;
}
#[no_mangle]
pub unsafe extern "C" fn car(mut cell: *mut CELL) -> *mut CELL {
    if !cell.is_null() && (*cell).tag == 0 as libc::c_int {
        return (*cell).value.cons.car as *mut CELL
    } else {
        if cell.is_null() {
            return 0 as *mut libc::c_void as *mut CELL
        } else {
            s_print(stderr, cell);
            fprintf(stderr, b"is not list. in <car>\n\x00" as *const u8 as *const libc::c_char);
            error_in_lisp();
        }
    }
    return 0 as *mut libc::c_void as *mut CELL;
}
#[no_mangle]
pub unsafe extern "C" fn cdr(mut cell: *mut CELL) -> *mut CELL {
    if !cell.is_null() && (*cell).tag == 0 as libc::c_int {
        return (*cell).value.cons.cdr as *mut CELL
    } else {
        if cell.is_null() {
            return 0 as *mut libc::c_void as *mut CELL
        } else {
            s_print(stderr, cell);
            fprintf(stderr,
                    b"is not list. in <cdr>\n\x00" as *const u8 as
                        *const libc::c_char);
            error_in_lisp();
        }
    }
    return 0 as *mut libc::c_void as *mut CELL;
}
#[no_mangle]
pub unsafe extern "C" fn equal(mut x: *mut libc::c_void,
                               mut y: *mut libc::c_void) -> libc::c_int {
    return if x == y {
        (0 as libc::c_int == 0) as libc::c_int
    } else if x.is_null() || y.is_null() {
        0 as libc::c_int
    } else if (*(x as *mut CELL)).tag != (*(y as *mut CELL)).tag {
        0 as libc::c_int
    } else if (*(x as *mut CELL)).tag == 1 as libc::c_int {
        (strcmp((*(x as *mut CELL)).value.atom as *const libc::c_char,
                (*(y as *mut CELL)).value.atom as *const libc::c_char)
            == 0) as libc::c_int
    } else if (*(x as *mut CELL)).tag == 0 as libc::c_int {
        (equal((*(x as *mut CELL)).value.cons.car,
               (*(y as *mut CELL)).value.cons.car) != 0 &&
            equal((*(x as *mut CELL)).value.cons.cdr,
                  (*(y as *mut CELL)).value.cons.cdr) != 0) as
            libc::c_int
    } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn length(mut list: *mut CELL) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !list.is_null() && (*list).tag == 0 as libc::c_int {
        list = (*list).value.cons.cdr as *mut CELL;
        i += 1
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn ifnextchar(mut fp: *mut FILE, mut i: libc::c_int)
 -> libc::c_int {
    let mut c: libc::c_int = 0;
    loop  {
        c = my_getc.expect("non-null function pointer")(fp);
        if c == '\n' as i32 { LineNo += 1 }
        if !(c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 ||
                 c == '\r' as i32) {
            break ;
        }
    }
    if c == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    return if i == c {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        my_ungetc.expect("non-null function pointer")(c, fp);
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn comment(mut fp: *mut FILE) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = ifnextchar(fp, ';' as i32);
    if n == (0 as libc::c_int == 0) as libc::c_int {
        while my_getc.expect("non-null function pointer")(fp) != '\n' as i32
                  && feof(fp) == 0 {
        }
        LineNo += 1;
        comment(fp);
    } else { (n) == -(1 as libc::c_int); }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn s_read(mut fp: *mut FILE) -> *mut CELL {
    let mut n: libc::c_int = 0;
    n = ifnextchar(fp, '(' as i32);
    if n == (0 as libc::c_int == 0) as libc::c_int {
        return s_read_car(fp)
    } else { if n == 0 as libc::c_int { return s_read_atom(fp) } }
    if n == -(1 as libc::c_int) {
        error_in_lisp();
    } else { error_in_program(); }
    return 0 as *mut libc::c_void as *mut CELL;
}
#[no_mangle]
pub unsafe extern "C" fn myscanf(mut fp: *mut FILE,
                                 mut cp: *mut libc::c_uchar) -> libc::c_int {
    let mut code: libc::c_int = 0;
    code = my_getc.expect("non-null function pointer")(fp);
    if dividing_code_p(code) != 0 {
        return 0 as libc::c_int
    } else if code == -(1 as libc::c_int) {
        return -(1 as libc::c_int)
    } else if code == '\"' as i32 {
        let fresh0 = cp;
        cp = cp.offset(1);
        *fresh0 = code as libc::c_uchar;
        loop  {
            code = my_getc.expect("non-null function pointer")(fp);
            if code == -(1 as libc::c_int) {
                error_in_lisp();
            } else if code == '\"' as i32 {
                let fresh1 = cp;
                cp = cp.offset(1);
                *fresh1 = code as libc::c_uchar;
                let fresh2 = cp;
                cp = cp.offset(1);
                *fresh2 = '\u{0}' as i32 as libc::c_uchar;
                return 1 as libc::c_int
            } else {
                if code == '\\' as i32 {
                    let fresh3 = cp;
                    cp = cp.offset(1);
                    *fresh3 = code as libc::c_uchar;
                    code = my_getc.expect("non-null function pointer")(fp);
                    if code == -(1 as libc::c_int) { error_in_lisp(); }
                    let fresh4 = cp;
                    cp = cp.offset(1);
                    *fresh4 = code as libc::c_uchar
                } else {
                    let fresh5 = cp;
                    cp = cp.offset(1);
                    *fresh5 = code as libc::c_uchar
                }
            }
        }
    } else {
        let fresh6 = cp;
        cp = cp.offset(1);
        *fresh6 = code as libc::c_uchar;
        if code == '\\' as i32 {
            *cp.offset(-(1 as libc::c_int as isize)) =
                my_getc.expect("non-null function pointer")(fp) as
                    libc::c_uchar
        }
        loop  {
            code = my_getc.expect("non-null function pointer")(fp);
            if dividing_code_p(code) != 0 || code == -(1 as libc::c_int) {
                let fresh7 = cp;
                cp = cp.offset(1);
                *fresh7 = '\u{0}' as i32 as libc::c_uchar;
                my_ungetc.expect("non-null function pointer")(code, fp);
                return 1 as libc::c_int
            } else {
                let fresh8 = cp;
                cp = cp.offset(1);
                *fresh8 = code as libc::c_uchar;
                if code == '\\' as i32 {
                    *cp.offset(-(1 as libc::c_int as isize)) =
                        my_getc.expect("non-null function pointer")(fp) as
                            libc::c_uchar
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn s_read_atom(mut fp: *mut FILE) -> *mut CELL {
    let mut cell: *mut CELL = 0 as *mut CELL;
    let mut c: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut n: libc::c_int = 0;
    /* #ifdef _WIN32
    char *eucstr;
    #endif */
    comment(fp);
    /* changed by kurohashi.

     if (((n = fscanf(fp, SCANATOM, Buffer)) == 0) || (n == EOF)) {
	  error_in_lisp();
     }
     */
    n = myscanf(fp, Buffer.as_mut_ptr());
    if n == 0 as libc::c_int || n == -(1 as libc::c_int) { error_in_lisp(); }
    /* #ifdef _WIN32
	eucstr = toStringEUC(Buffer);
	strcpy(Buffer, eucstr);
	free(eucstr);
	#endif */
    if strcmp(Buffer.as_mut_ptr() as *const libc::c_char,
              b"NIL\x00" as *const u8 as *const libc::c_char) == 0 {
        cell = 0 as *mut libc::c_void as *mut CELL
    } else {
        cell =
            cons(0 as *mut libc::c_void as *mut CELL as *mut libc::c_void,
                 0 as *mut libc::c_void as *mut CELL as *mut libc::c_void);
        (*cell).tag = 1 as libc::c_int;
        c =
            lisp_alloc((::std::mem::size_of::<libc::c_uchar>() as
                            libc::c_ulong).wrapping_mul(strlen(Buffer.as_mut_ptr()
                                                                   as
                                                                   *const libc::c_char).wrapping_add(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_ulong))
                           as libc::c_int) as *mut libc::c_uchar;
        strcpy(c as *mut libc::c_char,
               Buffer.as_mut_ptr() as *const libc::c_char);
        (*cell).value.atom = c
    }
    return cell;
}
#[no_mangle]
pub unsafe extern "C" fn s_read_car(mut fp: *mut FILE) -> *mut CELL {
    let mut cell: *mut CELL = 0 as *mut CELL;
    let mut n: libc::c_int = 0;
    comment(fp);
    n = ifnextchar(fp, ')' as i32);
    if n == (0 as libc::c_int == 0) as libc::c_int {
        cell = 0 as *mut libc::c_void as *mut CELL;
        return cell
    } else {
        if n == 0 as libc::c_int {
            cell =
                cons(0 as *mut libc::c_void as *mut CELL as *mut libc::c_void,
                     0 as *mut libc::c_void as *mut CELL as
                         *mut libc::c_void);
            (*cell).value.cons.car = s_read(fp) as *mut libc::c_void;
            (*cell).value.cons.cdr = s_read_cdr(fp) as *mut libc::c_void;
            return cell
        }
    }
    if n == -(1 as libc::c_int) {
        error_in_lisp();
    } else { error_in_program(); }
    return 0 as *mut libc::c_void as *mut CELL;
}
#[no_mangle]
pub unsafe extern "C" fn s_read_cdr(mut fp: *mut FILE) -> *mut CELL {
    let mut cell: *mut CELL = 0 as *mut CELL;
    let mut n: libc::c_int = 0;
    comment(fp);
    n = ifnextchar(fp, ')' as i32);
    if n == (0 as libc::c_int == 0) as libc::c_int {
        cell = 0 as *mut libc::c_void as *mut CELL;
        return cell
    } else { if n == 0 as libc::c_int { cell = s_read_car(fp); return cell } }
    if n == -(1 as libc::c_int) {
        error_in_lisp();
    } else { error_in_program(); }
    return 0 as *mut libc::c_void as *mut CELL;
}
/*
------------------------------------------------------------------------------
	FUNCTION
	<assoc>:
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn assoc(mut item: *mut CELL, mut alist: *mut CELL)
 -> *mut CELL {
    while equal(item as *mut libc::c_void,
                car(car(alist)) as *mut libc::c_void) == 0 && !alist.is_null()
          {
        alist = cdr(alist)
    }
    return car(alist);
}
/*
------------------------------------------------------------------------------
	PROCEDURE
	<s_print>: pretty print S-expression
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn s_print(mut fp: *mut FILE, mut cell: *mut CELL)
 -> *mut CELL {
    _s_print_(fp, cell);
    fputc('\n' as i32, fp);
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn _s_print_(mut fp: *mut FILE, mut cell: *mut CELL)
 -> *mut CELL {
    if cell.is_null() {
        fprintf(fp, b"%s\x00" as *const u8 as *const libc::c_char,
                b"NIL\x00" as *const u8 as *const libc::c_char);
    } else {
        match (*cell).tag {
            0 => {
                fprintf(fp, b"%c\x00" as *const u8 as *const libc::c_char,
                        '(' as i32);
                _s_print_(fp, (*cell).value.cons.car as *mut CELL);
                _s_print_cdr(fp, (*cell).value.cons.cdr as *mut CELL);
                fprintf(fp, b"%c\x00" as *const u8 as *const libc::c_char,
                        ')' as i32);
            }
            1 => {
                fprintf(fp, b"%s\x00" as *const u8 as *const libc::c_char,
                        (*cell).value.atom);
            }
            _ => {
                error(OtherError as libc::c_int,
                      b"Illegal cell(in s_print)\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      -(1 as libc::c_int) as *mut libc::c_char);
            }
        }
    }
    return cell;
}
#[no_mangle]
pub unsafe extern "C" fn _s_print_cdr(mut fp: *mut FILE, mut cell: *mut CELL)
 -> *mut CELL {
    if !cell.is_null() {
        if !cell.is_null() && (*cell).tag == 0 as libc::c_int {
            fprintf(fp, b" \x00" as *const u8 as *const libc::c_char);
            _s_print_(fp, (*cell).value.cons.car as *mut CELL);
            _s_print_cdr(fp, (*cell).value.cons.cdr as *mut CELL);
        } else { fputc(' ' as i32, fp); _s_print_(fp, cell); }
    }
    return cell;
}
/*
------------------------------------------------------------------------------
	PROCEDURE			by yamaji
	<lisp_alloc>: あらかじめ一定領域を確保しておいて malloc を行う
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn lisp_alloc(mut n: libc::c_int) -> *mut libc::c_void {
    let mut tbl: *mut CELLTABLE = 0 as *mut CELLTABLE;
    let mut p: *mut CELL = 0 as *mut CELL;
    if (n as
            libc::c_ulong).wrapping_rem(::std::mem::size_of::<CELL>() as
                                            libc::c_ulong) != 0 {
        n =
            (n as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<CELL>() as
                                                 libc::c_ulong).wrapping_add(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                as libc::c_int
    } else {
        n =
            (n as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<CELL>() as
                                                 libc::c_ulong) as libc::c_int
                as libc::c_int
    }
    if CellTbl.is_null() ||
           !CellTbl.is_null() && (*CellTbl).n + n > (*CellTbl).max {
        /* 新たに一定領域を確保 */
        if !CellTbl.is_null() && !(*CellTbl).next.is_null() {
            CellTbl = (*CellTbl).next as *mut CELLTABLE;
            (*CellTbl).n = 0 as libc::c_int
        } else {
            tbl =
                my_alloc(::std::mem::size_of::<CELLTABLE>() as libc::c_ulong
                             as libc::c_int) as *mut CELLTABLE;
            (*tbl).cell =
                my_alloc((::std::mem::size_of::<CELL>() as
                              libc::c_ulong).wrapping_mul(16384 as libc::c_int
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int) as *mut CELL;
            (*tbl).pre = CellTbl as *mut libc::c_void;
            (*tbl).next = 0 as *mut libc::c_void;
            (*tbl).max = 16384 as libc::c_int;
            (*tbl).n = 0 as libc::c_int;
            if !CellTbl.is_null() {
                (*CellTbl).next = tbl as *mut libc::c_void
            }
            CellTbl = tbl
        }
    }
    p = (*CellTbl).cell.offset((*CellTbl).n as isize);
    (*CellTbl).n += n;
    if (*CellTbl).n > (*CellTbl).max { error_in_lisp(); }
    return p as *mut libc::c_void;
}
/*
------------------------------------------------------------------------------
	PROCEDURE			by yamaji
	<lisp_alloc_push>: 現在のメモリアロケート状態を記憶する
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn lisp_alloc_push() { CellTbl_save = *CellTbl; }
/*
------------------------------------------------------------------------------
	PROCEDURE			by yamaji
	<lisp_alloc_pop>: 記憶したメモリアロケート状態に戻す
------------------------------------------------------------------------------
*/
#[no_mangle]
pub unsafe extern "C" fn lisp_alloc_pop() {
    (*CellTbl).cell = CellTbl_save.cell;
    (*CellTbl).n = CellTbl_save.n;
}

pub unsafe extern "C" fn dividing_code_p(code: libc::c_int) -> libc::c_int {
    return match code as char {
        '\n' => 1,
        '\t' => 1,
        ';' => 1,
        '(' => 1,
        ')' => 1,
        ' ' => 1,
        _ => 0
    }
}