#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int)
     -> __off64_t;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t)
     -> ssize_t;
    #[no_mangle]
    fn cdb_hash(buf: *const libc::c_void, len: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn cdb_unpack(buf: *const libc::c_uchar) -> libc::c_uint;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
}
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
/* cdb_seek.c: old interface for reading cdb file
 *
 * This file is a part of tinycdb package by Michael Tokarev, mjt@corpit.ru.
 * Public domain.
 */
/* read a chunk from file, ignoring interrupts (EINTR) */
#[no_mangle]
pub unsafe extern "C" fn cdb_bread(mut fd: libc::c_int,
                                   mut buf: *mut libc::c_void,
                                   mut len: libc::c_int) -> libc::c_int {
    let mut l: libc::c_int = 0;
    while len > 0 as libc::c_int {
        loop  {
            l = read(fd, buf, len as size_t) as libc::c_int;
            if !(l < 0 as libc::c_int &&
                     *__errno_location() == 4 as libc::c_int) {
                break ;
            }
        }
        if l <= 0 as libc::c_int {
            if l == 0 { *__errno_location() = 5 as libc::c_int }
            return -(1 as libc::c_int)
        }
        buf =
            (buf as *mut libc::c_char).offset(l as isize) as
                *mut libc::c_void;
        len -= l
    }
    return 0 as libc::c_int;
}
/* find a given key in cdb file, seek a file pointer to it's value and
   place data length to *dlenp. */
#[no_mangle]
pub unsafe extern "C" fn cdb_seek(mut fd: libc::c_int,
                                  mut key: *const libc::c_void,
                                  mut klen: libc::c_uint,
                                  mut dlenp: *mut libc::c_uint)
 -> libc::c_int {
    let mut htstart: libc::c_uint = 0; /* hash table start position */
    let mut htsize: libc::c_uint = 0; /* number of elements in a hash table */
    let mut httodo: libc::c_uint = 0; /* hash table elements left to look */
    let mut hti: libc::c_uint = 0; /* hash table index */
    let mut pos: libc::c_uint = 0; /* position in a file */
    let mut hval: libc::c_uint = 0; /* key's hash value */
    let mut rbuf: [libc::c_uchar; 64] = [0; 64]; /* read buffer */
    let mut needseek: libc::c_int =
        1 as libc::c_int; /* if we should seek to a hash slot */
    hval = cdb_hash(key, klen); /* position in TOC */
    pos = (hval & 0xff as libc::c_int as libc::c_uint) << 3 as libc::c_int;
    /* read the hash table parameters */
    if lseek(fd, pos as __off64_t, 0 as libc::c_int) <
           0 as libc::c_int as libc::c_long ||
           cdb_bread(fd, rbuf.as_mut_ptr() as *mut libc::c_void,
                     8 as libc::c_int) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    } /* start position in hash table */
    htsize =
        cdb_unpack(rbuf.as_mut_ptr().offset(4 as libc::c_int as isize) as
                       *const libc::c_uchar);
    if htsize == 0 as libc::c_int as libc::c_uint { return 0 as libc::c_int }
    hti = (hval >> 8 as libc::c_int).wrapping_rem(htsize);
    httodo = htsize;
    htstart = cdb_unpack(rbuf.as_mut_ptr() as *const libc::c_uchar);
    loop  {
        if needseek != 0 &&
               lseek(fd,
                     htstart.wrapping_add(hti << 3 as libc::c_int) as
                         __off64_t, 0 as libc::c_int) <
                   0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int)
        }
        if cdb_bread(fd, rbuf.as_mut_ptr() as *mut libc::c_void,
                     8 as libc::c_int) < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
        pos =
            cdb_unpack(rbuf.as_mut_ptr().offset(4 as libc::c_int as isize) as
                           *const libc::c_uchar);
        if pos == 0 as libc::c_int as libc::c_uint {
            /* not found */
            return 0 as libc::c_int
        }
        if cdb_unpack(rbuf.as_mut_ptr() as *const libc::c_uchar) != hval {
            /* hash value not matched */
            needseek = 0 as libc::c_int
        } else {
            /* hash value matched */
            if lseek(fd, pos as __off64_t, 0 as libc::c_int) <
                   0 as libc::c_int as libc::c_long ||
                   cdb_bread(fd, rbuf.as_mut_ptr() as *mut libc::c_void,
                             8 as libc::c_int) < 0 as libc::c_int {
                return -(1 as libc::c_int)
            }
            if cdb_unpack(rbuf.as_mut_ptr() as *const libc::c_uchar) == klen {
                /* we're looked to other place, should seek back */
                /* key length matches */
                /* read the key from file and compare with wanted */
                let mut l: libc::c_uint = klen; /* save value length */
                let mut c: libc::c_uint = 0;
                let mut k: *const libc::c_char = key as *const libc::c_char;
                if !dlenp.is_null() {
                    *dlenp =
                        cdb_unpack(rbuf.as_mut_ptr().offset(4 as libc::c_int
                                                                as isize) as
                                       *const libc::c_uchar)
                }
                loop  {
                    if l == 0 {
                        /* the whole key read and matches, return */
                        return 1 as libc::c_int
                    }
                    c =
                        if l as libc::c_ulong >
                               ::std::mem::size_of::<[libc::c_uchar; 64]>() as
                                   libc::c_ulong {
                            ::std::mem::size_of::<[libc::c_uchar; 64]>() as
                                libc::c_ulong
                        } else { l as libc::c_ulong } as libc::c_uint;
                    if cdb_bread(fd, rbuf.as_mut_ptr() as *mut libc::c_void,
                                 c as libc::c_int) < 0 as libc::c_int {
                        return -(1 as libc::c_int)
                    }
                    if memcmp(rbuf.as_mut_ptr() as *const libc::c_void,
                              k as *const libc::c_void, c as libc::c_ulong) !=
                           0 as libc::c_int {
                        break ;
                    }
                    k = k.offset(c as isize);
                    l = l.wrapping_sub(c)
                }
            }
            needseek = 1 as libc::c_int
        }
        httodo = httodo.wrapping_sub(1);
        if httodo == 0 { return 0 as libc::c_int }
        hti = hti.wrapping_add(1);
        if hti == htsize {
            hti = 0 as libc::c_int as libc::c_uint;
            needseek = 1 as libc::c_int
        }
    };
}
