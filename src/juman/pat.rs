#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use crate::juman::ctools::{__fxstat, exit, fileno, malloc_pat_index_list, malloc_pat_node, mmap, strchr, strcmp, strcpy, strlen, strncmp, strtok};
use crate::juman::structs::{_dic_t, pat_index_list, pat_node, stat, timespec};
use crate::juman::types::{__off_t, caddr_t, FILE, size_t};

#[inline]
unsafe extern "C" fn fstat(mut __fd: libc::c_int, mut __statbuf: *mut stat) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[no_mangle]
pub static mut tree_top: [pat_node; 5] =
    [pat_node{il:
                  pat_index_list{next:
                                     0 as *const pat_index_list as
                                         *mut pat_index_list,
                                 index: 0,},
              checkbit: 0,
              right: 0 as *const pat_node as *mut pat_node,
              left: 0 as *const pat_node as *mut pat_node,}; 5];
#[no_mangle]
pub static mut dic_file: [*mut FILE; 5] = [0 as *const FILE as *mut FILE; 5];
static mut dicinfo: [_dic_t; 5] =
    [_dic_t{used: 0,
            fd: 0,
            size: 0,
            addr: 0 as *const libc::c_char as *mut libc::c_char,}; 5];
unsafe extern "C" fn pat_strcpy(mut s1: *mut libc::c_char,
                                mut s2: *mut libc::c_char)
 -> *mut libc::c_char {
    loop  {
        let fresh0 = s2;
        s2 = s2.offset(1);
        let fresh1 = s1;
        s1 = s1.offset(1);
        *fresh1 = *fresh0;
        if !(*fresh1 != 0) { break ; }
    }
    return s1.offset(-(1 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn pat_init_tree_top(mut ptr: *mut pat_node) {
    (*ptr).il.index = -(1 as libc::c_int) as libc::c_long;
    (*ptr).checkbit = -(1 as libc::c_int) as libc::c_short;
    (*ptr).right = ptr;
    (*ptr).left = ptr;
}
#[no_mangle]
pub unsafe extern "C" fn pat_search(mut f: *mut FILE,
                                    mut key: *mut libc::c_char,
                                    mut x_ptr: *mut pat_node,
                                    mut rslt: *mut libc::c_char)
 -> *mut pat_node {
    let mut ptr: *mut pat_node = 0 as *mut pat_node;
    let mut tmp_ptr: *mut pat_node = 0 as *mut pat_node;
    let mut top_ptr: *mut pat_node = x_ptr;
    let mut tmp_x_ptr: *mut pat_node = x_ptr;
    let mut in_hash: libc::c_int = 0 as libc::c_int;
    let mut tmp_l_ptr: *mut pat_index_list = 0 as *mut pat_index_list;
    // let mut i: libc::c_int = 0;
    let mut key_length: libc::c_int = strlen(key) as libc::c_int;
    let mut buffer: [libc::c_char; 50000] = [0; 50000];
    let mut totyu_match_len: libc::c_int = 0 as libc::c_int;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    rslt = rslt.offset(strlen(rslt) as isize);
    r = rslt;
    loop  {
        ptr = x_ptr;
        if (*ptr).checkbit as libc::c_int % 24 as libc::c_int ==
               0 as libc::c_int &&
               (*ptr).checkbit as libc::c_int != 0 as libc::c_int {
            tmp_x_ptr = ptr;
            loop  {
                tmp_ptr = tmp_x_ptr;
                tmp_x_ptr = (*tmp_x_ptr).left;
                if !(((*tmp_ptr).checkbit as libc::c_int) <
                         (*tmp_x_ptr).checkbit as libc::c_int) {
                    break ;
                }
            }
            in_hash =
                hash_check_proc(f, (*tmp_x_ptr).il.index,
                                buffer.as_mut_ptr());
            strtok(buffer.as_mut_ptr(),
                   b"\t\x00" as *const u8 as *const libc::c_char);
            if strncmp(key, buffer.as_mut_ptr(),
                       ((*ptr).checkbit as libc::c_int / 8 as libc::c_int) as
                           libc::c_ulong) == 0 as libc::c_int {
                totyu_match_len =
                    (*ptr).checkbit as libc::c_int / 8 as libc::c_int;
                tmp_l_ptr = &mut (*tmp_x_ptr).il;
                while !tmp_l_ptr.is_null() {
                    in_hash =
                        hash_check_proc(f, (*tmp_l_ptr).index,
                                        buffer.as_mut_ptr());
                    r = pat_strcpy(r, buffer.as_mut_ptr());
                    let fresh2 = r;
                    r = r.offset(1);
                    *fresh2 = '\n' as i32 as libc::c_char;
                    *r = '\u{0}' as i32 as libc::c_char;
                    tmp_l_ptr = (*tmp_l_ptr).next
                }
            } else { return x_ptr }
        }
        if pat_bits(key, (*x_ptr).checkbit as libc::c_int, key_length) ==
               1 as libc::c_int {
            x_ptr = (*x_ptr).right
        } else { x_ptr = (*x_ptr).left }
        if !(((*ptr).checkbit as libc::c_int) <
                 (*x_ptr).checkbit as libc::c_int) {
            break ;
        }
    }
    if tmp_x_ptr != x_ptr || top_ptr == x_ptr {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tmp_len: libc::c_int = 0;
        in_hash = hash_check_proc(f, (*x_ptr).il.index, buffer.as_mut_ptr());
        s = strchr(buffer.as_mut_ptr(), '\t' as i32);
        *s = '\u{0}' as i32 as libc::c_char;
        tmp_len =
            s.wrapping_offset_from(buffer.as_mut_ptr()) as libc::c_long as
                libc::c_int;
        if strncmp(key, buffer.as_mut_ptr(), tmp_len as libc::c_ulong) ==
               0 as libc::c_int {
            if totyu_match_len != key_length {
                tmp_l_ptr = &mut (*x_ptr).il;
                while !tmp_l_ptr.is_null() {
                    in_hash =
                        hash_check_proc(f, (*tmp_l_ptr).index,
                                        buffer.as_mut_ptr());
                    r = pat_strcpy(r, buffer.as_mut_ptr());
                    let fresh3 = r;
                    r = r.offset(1);
                    *fresh3 = '\n' as i32 as libc::c_char;
                    *r = '\u{0}' as i32 as libc::c_char;
                    tmp_l_ptr = (*tmp_l_ptr).next
                }
            }
        }
    }
    return x_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn pat_search_exact(mut f: *mut FILE,
                                          mut key: *mut libc::c_char,
                                          mut x_ptr: *mut pat_node,
                                          mut rslt: *mut libc::c_char)
 -> *mut pat_node {
    let mut ptr: *mut pat_node = 0 as *mut pat_node;
    // let mut tmp_ptr: *mut pat_node = 0 as *mut pat_node;
    // let mut top_ptr: *mut pat_node = x_ptr;
    // let mut tmp_x_ptr: *mut pat_node = x_ptr;
    let mut tmp_l_ptr: *mut pat_index_list = 0 as *mut pat_index_list;
    let mut in_hash: libc::c_int = 0;
    // let mut i: libc::c_int = 0;
    let mut key_length: libc::c_int = strlen(key) as libc::c_int;
    let mut buffer: [libc::c_char; 50000] = [0; 50000];
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    rslt = rslt.offset(strlen(rslt) as isize);
    r = rslt;
    loop  {
        ptr = x_ptr;
        if pat_bits(key, (*x_ptr).checkbit as libc::c_int, key_length) ==
               1 as libc::c_int {
            x_ptr = (*x_ptr).right
        } else { x_ptr = (*x_ptr).left }
        if !(((*ptr).checkbit as libc::c_int) <
                 (*x_ptr).checkbit as libc::c_int) {
            break ;
        }
    }
    in_hash = hash_check_proc(f, (*x_ptr).il.index, buffer.as_mut_ptr());
    strtok(buffer.as_mut_ptr(),
           b"\t\x00" as *const u8 as *const libc::c_char);
    if strcmp(key, buffer.as_mut_ptr()) == 0 as libc::c_int {
        tmp_l_ptr = &mut (*x_ptr).il;
        while !tmp_l_ptr.is_null() {
            in_hash =
                hash_check_proc(f, (*tmp_l_ptr).index, buffer.as_mut_ptr());
            r = pat_strcpy(r, buffer.as_mut_ptr());
            let fresh4 = r;
            r = r.offset(1);
            *fresh4 = '\n' as i32 as libc::c_char;
            *r = '\u{0}' as i32 as libc::c_char;
            tmp_l_ptr = (*tmp_l_ptr).next
        }
    }
    return x_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn pat_search4insert(mut key: *mut libc::c_char,
                                           mut x_ptr: *mut pat_node)
 -> *mut pat_node {
    let mut ptr: *mut pat_node = 0 as *mut pat_node;
    // let mut tmp_ptr: *mut pat_node = 0 as *mut pat_node;
    // let mut tmp_x_ptr: *mut pat_node = 0 as *mut pat_node;
    // let mut checked_char: libc::c_int = 0 as libc::c_int;
    let mut key_length: libc::c_int = strlen(key) as libc::c_int;
    loop  {
        ptr = x_ptr;
        if pat_bits(key, (*x_ptr).checkbit as libc::c_int, key_length) ==
               1 as libc::c_int {
            x_ptr = (*x_ptr).right
        } else { x_ptr = (*x_ptr).left }
        if !(((*ptr).checkbit as libc::c_int) <
                 (*x_ptr).checkbit as libc::c_int) {
            break ;
        }
    }
    return x_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn pat_insert(mut f: *mut FILE,
                                    mut line: *mut libc::c_char,
                                    mut index: libc::c_long,
                                    mut x_ptr: *mut pat_node,
                                    mut kugiri: *mut libc::c_char) {
    let mut t_ptr: *mut pat_node = 0 as *mut pat_node;
    let mut p_ptr: *mut pat_node = 0 as *mut pat_node;
    let mut new_ptr: *mut pat_node = 0 as *mut pat_node;
    let mut diff_bit: libc::c_int = 0;
    // let mut i: libc::c_int = 0;
    let mut new_l_ptr: *mut pat_index_list = 0 as *mut pat_index_list;
    let mut tmp_l_ptr: *mut pat_index_list = 0 as *mut pat_index_list;
    let mut mae_wo_sasu_ptr: *mut pat_index_list = 0 as *mut pat_index_list;
    let mut in_hash: libc::c_int = 0;
    let mut buffer_length: libc::c_int = 0;
    let mut key_length: libc::c_int = 0;
    let mut key: [libc::c_char; 1000] = [0; 1000];
    let mut buffer: [libc::c_char; 50000] = [0; 50000];
    strcpy(key.as_mut_ptr(), line);
    strtok(key.as_mut_ptr(), kugiri);
    key_length = strlen(key.as_mut_ptr()) as libc::c_int;
    t_ptr = pat_search4insert(key.as_mut_ptr(), x_ptr);
    if (*t_ptr).il.index >= 0 as libc::c_int as libc::c_long {
        in_hash = hash_check_proc(f, (*t_ptr).il.index, buffer.as_mut_ptr());
        if strncmp(key.as_mut_ptr(), buffer.as_mut_ptr(),
                   strlen(key.as_mut_ptr())) == 0 as libc::c_int {
            tmp_l_ptr = &mut (*t_ptr).il;
            while !tmp_l_ptr.is_null() {
                in_hash =
                    hash_check_proc(f, (*tmp_l_ptr).index,
                                    buffer.as_mut_ptr());
                if strcmp(buffer.as_mut_ptr(), line) == 0 as libc::c_int {
                    return
                }
                mae_wo_sasu_ptr = tmp_l_ptr;
                tmp_l_ptr = (*tmp_l_ptr).next
            }
            new_l_ptr = malloc_pat_index_list();
            (*new_l_ptr).index = index;
            (*new_l_ptr).next = 0 as *mut pat_index_list;
            (*mae_wo_sasu_ptr).next = new_l_ptr;
            return
        }
    } else {
        *buffer.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
        *buffer.as_mut_ptr().offset(1 as libc::c_int as isize) =
            '\u{0}' as i32 as libc::c_char
    }
    buffer_length = strlen(buffer.as_mut_ptr()) as libc::c_int;
    diff_bit = 0 as libc::c_int;
    while pat_bits(key.as_mut_ptr(), diff_bit, key_length) ==
              pat_bits(buffer.as_mut_ptr(), diff_bit, buffer_length) {
        diff_bit += 1
    }
    loop  {
        p_ptr = x_ptr;
        if pat_bits(key.as_mut_ptr(), (*x_ptr).checkbit as libc::c_int,
                    key_length) == 1 as libc::c_int {
            x_ptr = (*x_ptr).right
        } else { x_ptr = (*x_ptr).left }
        if !(((*x_ptr).checkbit as libc::c_int) < diff_bit &&
                 ((*p_ptr).checkbit as libc::c_int) <
                     (*x_ptr).checkbit as libc::c_int) {
            break ;
        }
    }
    new_ptr = malloc_pat_node();
    (*new_ptr).checkbit = diff_bit as libc::c_short;
    (*new_ptr).il.index = index;
    (*new_ptr).il.next = 0 as *mut pat_index_list;
    if pat_bits(key.as_mut_ptr(), (*new_ptr).checkbit as libc::c_int,
                key_length) == 1 as libc::c_int {
        (*new_ptr).right = new_ptr;
        (*new_ptr).left = x_ptr
    } else { (*new_ptr).left = new_ptr; (*new_ptr).right = x_ptr }
    if pat_bits(key.as_mut_ptr(), (*p_ptr).checkbit as libc::c_int,
                key_length) == 1 as libc::c_int {
        (*p_ptr).right = new_ptr
    } else { (*p_ptr).left = new_ptr };
}
#[no_mangle]
pub unsafe extern "C" fn pat_bits(mut string: *mut libc::c_char,
                                  mut cbit: libc::c_int, mut len: libc::c_int)
 -> libc::c_int {
    let mut moji_idx: libc::c_int = cbit / 8 as libc::c_int;
    let mut moji: libc::c_char = *string.offset(moji_idx as isize);
    let mut idx_in_moji: libc::c_int = cbit % 8 as libc::c_int;
    if cbit == -(1 as libc::c_int) { return 1 as libc::c_int }
    if (len - 1 as libc::c_int) < moji_idx { return 0 as libc::c_int }
    return ((moji as libc::c_int) << idx_in_moji & 0x80 as libc::c_int) >>
               7 as libc::c_int;
}
/* ***************************************************
* get_line --- ファイルの pos 文字目から \n まで読む
* 
* パラメータ
*   f --- 読むファイル
*   pos --- 読み込み始める位置
*   buf --- 読み込むバッファ
* 
* 返し値
*   文字数(strlen方式) 
*   -1 : 失敗
****************************************************/
#[no_mangle]
pub unsafe extern "C" fn get_line(mut f: *mut FILE, mut pos: libc::c_long)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int;
    // let mut j: libc::c_int = 0 as libc::c_int;
    // let mut ch: libc::c_int = 0;
    let mut ffd: libc::c_int = fileno(f);
    static mut oldf: libc::c_int = -(1 as libc::c_int);
    static mut addr: caddr_t = 0 as *const libc::c_char as *mut libc::c_char;
    static mut size: libc::off_t = 0;
    let mut st: stat =
        stat{st_dev: 0,
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
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if oldf != ffd {
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            if ffd == dicinfo[i as usize].fd && dicinfo[i as usize].used != 0
               {
                oldf = dicinfo[i as usize].fd;
                addr = dicinfo[i as usize].addr;
                size = dicinfo[i as usize].size;
                break ;
            } else if dicinfo[i as usize].used == 0 as libc::c_int {
                dicinfo[i as usize].fd = ffd;
                dicinfo[i as usize].used = 1 as libc::c_int;
                fstat(dicinfo[i as usize].fd, &mut st);
                size = st.st_size;
                dicinfo[i as usize].size = size;
                addr =
                    mmap(0 as *mut libc::c_void,
                         dicinfo[i as usize].size as size_t,
                         0x1 as libc::c_int, 0x2 as libc::c_int,
                         dicinfo[i as usize].fd, 0 as libc::c_int as __off_t)
                        as caddr_t;
                dicinfo[i as usize].addr = addr;
                break ;
            } else { i += 1 }
        }
        if i == 5 as libc::c_int { exit(1 as libc::c_int); }
        oldf = ffd
    }
    if pos >= size { return 0 as *mut libc::c_char }
    return addr.offset(pos as isize);
}
/* ***************************************************
* show_pat --- パトリシア木データを出力
*
* パラメータ
*   top_ptr --- 検索開始ノードの位置(ポインタ)
*   out_to --- 出力先(stdoutやファイル)
* 
* 返し値
*   無し。パトリシア木データを出力。
****************************************************/
#[no_mangle]
pub unsafe extern "C" fn show_pat(mut top_ptr: *mut pat_node, mut out_to: *mut FILE, mut prefix: *mut libc::c_char) {
}


pub unsafe extern "C" fn hash_check_proc(mut f: *mut FILE, mut index: libc::c_long, mut buf: *mut libc::c_char) -> libc::c_int {
    // #[cfg(USE_HASH)]
    strcpy(buf, get_line(f, index));
    // #[cfg(USE_HASH)]
    return 0;

    // #[cfg(not(USE_HASH))]
    // let mut data = th_hash_out(hash_array, HASH_SIZE, index, f);
    // if data.is_null() {
    //     strcpy(buf, get_line(f, index));
    //     th_hash_in(hash_array, HASH_SIZE, index, buf, f);
    //     0
    // } else {
    //     strcpy(buf, data);
    //     1
    // }
}