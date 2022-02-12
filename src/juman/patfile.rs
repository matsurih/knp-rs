#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::juman::ctools::{close, exit, fprintf, malloc, open, printf, read, stderr, write};
use crate::juman::structs::{_IO_FILE, pat_index_list, pat_node};
use crate::juman::types::size_t;

#[no_mangle]
pub static mut fd_pat: libc::c_int = 0;
/* パト木のセーブ/ロード用ファイルディスクリプタ */
/* *****************************************************
* com_l --- 木のロード
*  by 米沢恵司(keiji-y@is.aist-nara.ac.jp)
*
* パラメータ、返し値
*   なし
******************************************************/
#[no_mangle]
pub unsafe extern "C" fn com_l(mut fname_pat: *mut libc::c_char,
                               mut ptr: *mut pat_node) {
    /*  fprintf(stderr, "# Loading pat-tree \"%s\" ... ",fname_pat); */
    fd_pat = open(fname_pat, 0 as libc::c_int | 0 as libc::c_int);
    if fd_pat == -(1 as libc::c_int) {
        fprintf(stderr,
                b"\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab %s \xe3\x81\x8c\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x97\xe3\x83\xb3\xe5\x87\xba\xe6\x9d\xa5\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\xe3\x80\x82\n\x00"
                    as *const u8 as *const libc::c_char, fname_pat);
        exit(1 as libc::c_int);
    }
    (*ptr).right = load_anode(ptr);
    close(fd_pat);
    /*  fprintf(stderr,"done.\n"); */
}
/* ***************************************************
* load_anode --- パトリシア木をロード
*  by 米沢恵司(keiji-y@is.aist-nara.ac.jp)
*
* パラメータ
*   in --- 入力先ファイル
*   p_ptr --- このノードが外部接点であった時にインデックスを格納する場所
*             内部接点であったときは、このポインタは右の子に渡される。
*
* アルゴリズム
*   チェックビットを読み込んだら、それは内部接点だから新しくノードを作る
*     左部分木、右部分木の順に再帰する
*     左再帰の時は新しく作ったこの接点のポインタを、
*     右再帰の時は p_ptr をインデックスの格納場所として渡す。
*   インデックスを読み込んだら、それは外部接点だから、p_ptr->index に格納
*
* メモ
*   インデックスの格納場所が元と違うが、特に問題ない。
*************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn malloc_pat_index_list() -> *mut pat_index_list {
    static mut idx: libc::c_int =
        1024 as
            libc::c_int; /* 新しく作ったノード(==このノード)を指すポインタ */
    static mut ptr: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    if idx == 1024 as libc::c_int {
        ptr =
            malloc((::std::mem::size_of::<pat_index_list>() as
                        libc::c_ulong).wrapping_mul(idx as libc::c_ulong)) as
                *mut libc::c_char;
        idx = 0 as libc::c_int
    }
    let fresh0 = idx;
    idx = idx + 1;
    return ptr.offset((::std::mem::size_of::<pat_index_list>() as
                           libc::c_ulong).wrapping_mul(fresh0 as
                                                           libc::c_ulong) as
                          isize) as *mut pat_index_list;
}
#[no_mangle]
pub unsafe extern "C" fn malloc_pat_node() -> *mut pat_node {
    static mut idx: libc::c_int = 1024 as libc::c_int;
    static mut ptr: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    if idx == 1024 as libc::c_int {
        ptr =
            malloc((::std::mem::size_of::<pat_node>() as
                        libc::c_ulong).wrapping_mul(idx as libc::c_ulong)) as
                *mut libc::c_char;
        idx = 0 as libc::c_int
    }
    let fresh1 = idx;
    idx = idx + 1;
    return ptr.offset((::std::mem::size_of::<pat_node>() as
                           libc::c_ulong).wrapping_mul(fresh1 as
                                                           libc::c_ulong) as
                          isize) as *mut pat_node;
}
#[no_mangle]
pub unsafe extern "C" fn load_anode(mut p_ptr: *mut pat_node)
 -> *mut pat_node {
    let mut c: libc::c_uchar = 0;
    let mut new_ptr: *mut pat_node = 0 as *mut pat_node;
    let mut tmp_idx: libc::c_long = 0;
    let mut new_l_ptr: *mut pat_index_list = 0 as *mut pat_index_list;
    let mut t_ptr: *mut pat_index_list = 0 as *mut pat_index_list;
    c = egetc(fd_pat);
    return if c as libc::c_int & 0x80 as libc::c_int != 0 {
        /* 葉っぱの処理、インデックスの読み込み */
        while c as libc::c_int & 0x80 as libc::c_int != 0 {
            tmp_idx =
                ((c as libc::c_int & 0x3f as libc::c_int) <<
                    24 as libc::c_int) as libc::c_long;
            tmp_idx |=
                ((egetc(fd_pat) as libc::c_int) << 16 as libc::c_int) as
                    libc::c_long;
            tmp_idx |=
                ((egetc(fd_pat) as libc::c_int) << 8 as libc::c_int) as
                    libc::c_long;
            tmp_idx |= egetc(fd_pat) as libc::c_long;
            if (*p_ptr).il.index < 0 as libc::c_int as libc::c_long {
                new_l_ptr = &mut (*p_ptr).il
            } else {
                new_l_ptr = malloc_pat_index_list();
                (*t_ptr).next = new_l_ptr
            }
            (*new_l_ptr).index = tmp_idx;
            (*new_l_ptr).next = 0 as *mut pat_index_list;
            t_ptr = new_l_ptr;
            if c as libc::c_int & 0x40 as libc::c_int != 0 { break; }
            c = egetc(fd_pat)
        }
        p_ptr
    } else {
        /* 内部接点の処理、再帰する */
        new_ptr = malloc_pat_node(); /* チェックビット */
        (*new_ptr).checkbit =
            (((c as libc::c_int) << 8 as libc::c_int |
                egetc(fd_pat) as libc::c_int) - 1 as libc::c_int) as
                libc::c_short;
        /*    printf("#cb %d\n",new_ptr->checkbit);*/
        (*new_ptr).il.index = -(1 as libc::c_int) as libc::c_long;
        (*new_ptr).left = load_anode(new_ptr);
        (*new_ptr).right = load_anode(p_ptr);
        new_ptr
    };
}
// Initialized in run_static_initializers
static mut ctr: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn egetc(mut file_discripter: libc::c_int)
 -> libc::c_uchar {
    static mut fd_pat_check: libc::c_int = -(1 as libc::c_int);
    static mut buf: [libc::c_char; 8192] = [0; 8192];
    if file_discripter != fd_pat_check {
        /* バッファの初期化 */
        fd_pat_check = file_discripter;
        ctr =
            (::std::mem::size_of::<[libc::c_char; 8192]>() as
                 libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong) as libc::c_int
    }
    ctr += 1;
    if ctr as libc::c_ulong ==
           ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong {
        ctr = 0 as libc::c_int;
        read(file_discripter, buf.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong);
        /* OL(.);fflush(stdout);*/
    }
    return buf[ctr as usize] as libc::c_uchar;
}
/* ****************************************************
* com_s --- 木のセーブ 
*  by 米沢恵司(keiji-y@is.aist-nara.ac.jp)
*
* パラメータ、返し値
*   なし
*****************************************************/
#[no_mangle]
pub unsafe extern "C" fn com_s(mut fname_pat: *mut libc::c_char,
                               mut ptr: *mut pat_node) {
    let mut i: libc::c_int = 0; /* ファイル出力 */
    printf(b"Saving pat-tree \"%s\" ...\n\x00" as *const u8 as
               *const libc::c_char, fname_pat); /* flush */
    fd_pat =
        open(fname_pat,
             0o1 as libc::c_int | 0o100 as libc::c_int | 0 as libc::c_int,
             0o644 as libc::c_int);
    if fd_pat == -(1 as libc::c_int) {
        fprintf(stderr,
                b"\xe3\x83\x95\xe3\x82\xa1\xe3\x82\xa4\xe3\x83\xab %s \xe3\x81\x8c\xe3\x82\xaa\xe3\x83\xbc\xe3\x83\x97\xe3\x83\xb3\xe5\x87\xba\xe6\x9d\xa5\xe3\x81\xbe\xe3\x81\x9b\xe3\x82\x93\xe3\x80\x82\n\x00"
                    as *const u8 as *const libc::c_char, fname_pat);
        exit(1 as libc::c_int);
    }
    save_pat((*ptr).right);
    i = 0 as libc::c_int;
    while i < 8192 as libc::c_int {
        eputc(0 as libc::c_int as libc::c_uchar, fd_pat);
        i += 1
    }
    close(fd_pat);
}
/* ***************************************************
* save_pat --- パトリシア木データをセーブ 
*  by 米沢恵司(keiji-y@is.aist-nara.ac.jp)
*
* パラメータ
*   top_ptr --- 検索開始ノードの位置(ポインタ)
*   out_to --- 出力先(stdoutやファイル)
* 
* 返し値
*   無し。パトリシア木データを出力。
*
* 出力フォーマット --- 8ビットに区切ってバイナリ出力
*   左優先探索で内部接点はチェックビット、外部接点はインデックスを出力
*   チェックビット --- 基本的にそのまま (第 0 ビットが 0)
*     ただし -1 のとき困るので 1 を足す
*   インデックス --- 第 0 ビットを 1 にする
****************************************************/
#[no_mangle]
pub unsafe extern "C" fn save_pat(mut top_ptr: *mut pat_node) {
    let mut ptr: *mut pat_index_list = 0 as *mut pat_index_list;
    // let mut out_idx: libc::c_long = 0;
    /* 内部接点の処理、チェックビットを出力 */
    eputc(((*top_ptr).checkbit as libc::c_int + 1 as libc::c_int >>
               8 as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar,
          fd_pat);
    eputc(((*top_ptr).checkbit as libc::c_int + 1 as libc::c_int &
               0xff as libc::c_int) as libc::c_uchar, fd_pat);
    /* 左右の Subtree の処理。葉っぱならインデックスを出力、
     葉っぱでなければ再帰。*/
    if ((*top_ptr).checkbit as libc::c_int) <
           (*(*top_ptr).left).checkbit as libc::c_int {
        save_pat((*top_ptr).left);
    } else {
        ptr = &mut (*(*top_ptr).left).il;
        if (*ptr).index < 0 as libc::c_int as libc::c_long {
            dummy();
        } else {
            while !ptr.is_null() {
                if (*ptr).next.is_null() {
                    eputc(((*ptr).index >> 24 as libc::c_int &
                               0x3f as libc::c_int as libc::c_long |
                               0xc0 as libc::c_int as libc::c_long) as
                              libc::c_uchar, fd_pat);
                } else {
                    eputc(((*ptr).index >> 24 as libc::c_int &
                               0x3f as libc::c_int as libc::c_long |
                               0x80 as libc::c_int as libc::c_long) as
                              libc::c_uchar, fd_pat);
                }
                eputc(((*ptr).index >> 16 as libc::c_int &
                           0xff as libc::c_int as libc::c_long) as
                          libc::c_uchar, fd_pat);
                eputc(((*ptr).index >> 8 as libc::c_int &
                           0xff as libc::c_int as libc::c_long) as
                          libc::c_uchar, fd_pat);
                eputc(((*ptr).index & 0xff as libc::c_int as libc::c_long) as
                          libc::c_uchar, fd_pat);
                ptr = (*ptr).next
            }
        }
    }
    if ((*top_ptr).checkbit as libc::c_int) <
           (*(*top_ptr).right).checkbit as libc::c_int {
        save_pat((*top_ptr).right);
    } else {
        ptr = &mut (*(*top_ptr).right).il;
        if (*ptr).index < 0 as libc::c_int as libc::c_long {
            dummy();
        } else {
            while !ptr.is_null() {
                if (*ptr).next.is_null() {
                    eputc(((*ptr).index >> 24 as libc::c_int &
                               0x3f as libc::c_int as libc::c_long |
                               0xc0 as libc::c_int as libc::c_long) as
                              libc::c_uchar, fd_pat);
                } else {
                    eputc(((*ptr).index >> 24 as libc::c_int &
                               0x3f as libc::c_int as libc::c_long |
                               0x80 as libc::c_int as libc::c_long) as
                              libc::c_uchar, fd_pat);
                }
                eputc(((*ptr).index >> 16 as libc::c_int &
                           0xff as libc::c_int as libc::c_long) as
                          libc::c_uchar, fd_pat);
                eputc(((*ptr).index >> 8 as libc::c_int &
                           0xff as libc::c_int as libc::c_long) as
                          libc::c_uchar, fd_pat);
                eputc(((*ptr).index & 0xff as libc::c_int as libc::c_long) as
                          libc::c_uchar, fd_pat);
                ptr = (*ptr).next
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dummy() {
    eputc(0xff as libc::c_int as libc::c_uchar, fd_pat);
    eputc(0xff as libc::c_int as libc::c_uchar, fd_pat);
    eputc(0xff as libc::c_int as libc::c_uchar, fd_pat);
    eputc(0xff as libc::c_int as libc::c_uchar, fd_pat);
}
#[no_mangle]
pub unsafe extern "C" fn eputc(mut c: libc::c_uchar,
                               mut file_discripter: libc::c_int) {
    static mut ctr_0: libc::c_int = 0 as libc::c_int;
    static mut buf: [libc::c_uchar; 8192] = [0; 8192];
    buf[ctr_0 as usize] = c as libc::c_char as libc::c_uchar;
    ctr_0 += 1;
    if ctr_0 == 8192 as libc::c_int {
        ctr_0 = 0 as libc::c_int;
        write(file_discripter, buf.as_mut_ptr(), 8192 as size_t);
    };
}
unsafe extern "C" fn run_static_initializers() {
    ctr =
        (::std::mem::size_of::<[libc::c_char; 8192]>() as
             libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
