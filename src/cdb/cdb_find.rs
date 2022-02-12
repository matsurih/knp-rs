#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn cdb_hash(buf: *const libc::c_void, len: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn cdb_unpack(buf: *const libc::c_uchar) -> libc::c_uint;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdb {
    pub cdb_fd: libc::c_int,
    pub cdb_fsize: libc::c_uint,
    pub cdb_dend: libc::c_uint,
    pub cdb_mem: *const libc::c_uchar,
    pub cdb_vpos: libc::c_uint,
    pub cdb_vlen: libc::c_uint,
    pub cdb_kpos: libc::c_uint,
    pub cdb_klen: libc::c_uint,
}
/* cdb_find.c: cdb_find routine
 *
 * This file is a part of tinycdb package by Michael Tokarev, mjt@corpit.ru.
 * Public domain.
 */
#[no_mangle]
pub unsafe extern "C" fn cdb_find(mut cdbp: *mut cdb,
                                  mut key: *const libc::c_void,
                                  mut klen: libc::c_uint) -> libc::c_int {
    let mut htp: *const libc::c_uchar =
        0 as *const libc::c_uchar; /* hash table pointer */
    let mut htab: *const libc::c_uchar =
        0 as *const libc::c_uchar; /* hash table */
    let mut htend: *const libc::c_uchar =
        0 as *const libc::c_uchar; /* end of hash table */
    let mut httodo: libc::c_uint = 0; /* ht bytes left to look */
    let mut pos: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut hval: libc::c_uint = 0;
    if klen >= (*cdbp).cdb_dend {
        /* if key size is too large */
        return 0 as libc::c_int
    }
    hval = cdb_hash(key, klen);
    /* find (pos,n) hash table to use */
  /* first 2048 bytes (toc) are always available */
  /* (hval % 256) * 8 */
    htp =
        (*cdbp).cdb_mem.offset((hval << 3 as libc::c_int &
                                    2047 as libc::c_int as libc::c_uint) as
                                   isize); /* index in toc (256x8) */
    n = cdb_unpack(htp.offset(4 as libc::c_int as isize)); /* table size */
    if n == 0 { /* not found */
        /* empty table */
        return 0 as libc::c_int
    } /* bytes of htab to lookup */
    httodo = n << 3 as libc::c_int; /* htab position */
    pos = cdb_unpack(htp);
    if n > (*cdbp).cdb_fsize >> 3 as libc::c_int || pos < (*cdbp).cdb_dend ||
           pos > (*cdbp).cdb_fsize ||
           httodo > (*cdbp).cdb_fsize.wrapping_sub(pos) {
        /* entrie htab within file ? */
        *__errno_location() = 71 as libc::c_int; /* htab pointer */
        return -(1 as libc::c_int)
    } /* after end of htab */
    htab = (*cdbp).cdb_mem.offset(pos as isize);
    htend = htab.offset(httodo as isize);
    /* htab starting position: rest of hval modulo htsize, 8bytes per elt */
    htp =
        htab.offset(((hval >> 8 as libc::c_int).wrapping_rem(n) <<
                         3 as libc::c_int) as isize); /* record position */
    loop  {
        pos = cdb_unpack(htp.offset(4 as libc::c_int as isize));
        if pos == 0 { return 0 as libc::c_int }
        if cdb_unpack(htp) == hval {
            if pos >
                   (*cdbp).cdb_dend.wrapping_sub(8 as libc::c_int as
                                                     libc::c_uint) {
                /* key+val lengths */
                *__errno_location() = 71 as libc::c_int;
                return -(1 as libc::c_int)
            }
            if cdb_unpack((*cdbp).cdb_mem.offset(pos as isize)) == klen {
                if (*cdbp).cdb_dend.wrapping_sub(klen) <
                       pos.wrapping_add(8 as libc::c_int as libc::c_uint) {
                    *__errno_location() = 71 as libc::c_int;
                    return -(1 as libc::c_int)
                }
                if memcmp(key,
                          (*cdbp).cdb_mem.offset(pos as
                                                     isize).offset(8 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                              as *const libc::c_void, klen as libc::c_ulong)
                       == 0 as libc::c_int {
                    n =
                        cdb_unpack((*cdbp).cdb_mem.offset(pos as
                                                              isize).offset(4
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize));
                    pos = pos.wrapping_add(8 as libc::c_int as libc::c_uint);
                    if (*cdbp).cdb_dend < n ||
                           (*cdbp).cdb_dend.wrapping_sub(n) <
                               pos.wrapping_add(klen) {
                        *__errno_location() = 71 as libc::c_int;
                        return -(1 as libc::c_int)
                    }
                    (*cdbp).cdb_kpos = pos;
                    (*cdbp).cdb_klen = klen;
                    (*cdbp).cdb_vpos = pos.wrapping_add(klen);
                    (*cdbp).cdb_vlen = n;
                    return 1 as libc::c_int
                }
            }
        }
        httodo = httodo.wrapping_sub(8 as libc::c_int as libc::c_uint);
        if httodo == 0 { return 0 as libc::c_int }
        htp = htp.offset(8 as libc::c_int as isize);
        if htp >= htend { htp = htab }
    };
}
