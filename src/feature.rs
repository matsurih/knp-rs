#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

use libc;

use crate::{atoi, Class, fprintf, free, PM_Memo, sprintf, sscanf, strcasecmp, strcat, strchr, strcmp, strcpy, strdup, strlen, strncmp, strstr};
use crate::ctools::{_sm_match_score, assign_sm, bgh_match_check, car, case2num, cdr, change_mrph, change_one_mrph_rep, check_auto_dic, check_nv_mi_parent_and_children, exit, get_scase_code, make_fukugoji_case_string, make_fukugoji_id, malloc, matched_ptr, set_pred_voice, sm2code, SM2CODEExist, sm_all_match, sm_match_check, stderr, subordinate_level_check, subordinate_level_comp, Thesaurus};
use crate::structs::{FEATURE_PATTERN, MRPH_DATA};
use crate::tools::{OptDisplay, OptExpress, OptInput, Options, OptPosModification};
use crate::types::{BNST_DATA, CELL, FEATURE, FEATUREptr, FILE, TAG_DATA};

#[no_mangle]
pub static mut EtcRuleArray: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut CurEtcRuleSize: libc::c_int = 0;
/*====================================================================

			     FEATURE処理

                                               S.Kurohashi 96. 7. 4

    $Id$
====================================================================*/
/*
  FEATUREの処理には次の３種類がある

  	(1) ファイル(S式または文字列) ==コピー==> ルール構造体

	(2) ルール構造体 ==付与==> 形態素または文節構造体
        	<○:□>は<○:…>というFEATUREへの上書き (なければ新規)
                <^○>は<○:…>の削除 (なければ無視)
		<&○>は関数呼出
			&表層:付与 -- 辞書引きによる表層格付与
			&表層:削除 -- すべての表層格削除
			&表層:○格 -- ○格付与
			&表層:^○格 -- ○格削除
			&MEMO:○ -- MEMOへの書き込み

	(3) ルール構造体 <==照合==> 形態素または文節構造体
	       	<○>は<○:…>というFEATUREがあればOK
	    	<^○>は<○:…>というFEATUREがなければOK
	    	<&○>は関数呼出
			&記英数カ -- 表記が記号,英文字,数字,カタカナ (形態素)
			&漢字 -- 表記が漢字 (形態素)
	    		&表層:○格 -- ○格がある (文節)
	    		&表層:照合 -- 係の表層格が受にある (係受)
			&D:n -- 構造体間が距離n以内 (係受)
			&レベル:強 -- 受が係以上 (係受)
			&レベル:l -- 自身がl以上 (係受)
			&係側:○ -- 係に○ (係受)

	※ プログラム内で形態素または文節構造体にFEATUREを与える
	場合は(2)のなかの assign_cfeature を用いる．

	※ プログラム内で形態素または文節構造体があるFEATUREを持つ
	かどうかを調べる場合は(3)のなかの check_feature を用いる．
*/
#[no_mangle]
pub static mut feature_buffer: [libc::c_char; 5120] = [0; 5120];

#[no_mangle]
pub unsafe extern "C" fn print_one_feature(mut cp: *mut libc::c_char, mut filep: *mut FILE) {
    if strncmp(cp, b"\xe4\xbb\xae\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char, strlen(b"\xe4\xbb\xae\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char)) == 0 {
        /* 仮付与したものを表示するとき用(-nbest) */
        if OptExpress == 16 as libc::c_int {
            fprintf(filep, b"\xef\xbc\x9c%s\xef\xbc\x9e\x00" as *const u8 as *const libc::c_char, cp.offset(strlen(b"\xe4\xbb\xae\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char) as isize));
        } else {
            fprintf(filep, b"<%s>\x00" as *const u8 as *const libc::c_char, cp.offset(strlen(b"\xe4\xbb\xae\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char) as isize));
        }
    } else if OptExpress == 16 as libc::c_int {
        fprintf(filep, b"\xef\xbc\x9c%s\xef\xbc\x9e\x00" as *const u8 as *const libc::c_char, cp);
    } else {
        fprintf(filep, b"<%s>\x00" as *const u8 as *const libc::c_char, cp);
    };
}

#[no_mangle]
pub unsafe extern "C" fn check_important_feature(mut fp: *mut FEATURE) -> libc::c_int {
    /* -simpletab でも出力する重要なfeature */
    return if comp_feature((*fp).cp, b"\xe7\x94\xa8\xe8\xa8\x80\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) != 0 ||
        comp_feature((*fp).cp, b"\xe4\xbd\x93\xe8\xa8\x80\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) != 0 ||
        comp_feature((*fp).cp, b"\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\xe7\xb5\x90\xe6\x9e\x9c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) != 0 ||
        comp_feature((*fp).cp, b"\xe8\xbf\xb0\xe8\xaa\x9e\xe9\xa0\x85\xe6\xa7\x8b\xe9\x80\xa0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) != 0 ||
        comp_feature((*fp).cp, b"Wikipedia\xe3\x82\xa8\xe3\x83\xb3\xe3\x83\x88\xe3\x83\xaa\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) != 0 ||
        comp_feature((*fp).cp, b"NE\xe5\x86\x85\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) != 0 ||
        comp_feature((*fp).cp, b"NE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) != 0 {
        (0 as libc::c_int == 0) as libc::c_int
    } else {
        0 as libc::c_int
    };
}

#[no_mangle]
pub unsafe extern "C" fn print_feature(mut fp: *mut FEATURE, mut filep: *mut FILE) {
    /* <f1><f2> ... <f3> という形式の出力 
       (ただしＴではじまるfeatureは表示しない) */
    while !fp.is_null() {
        if !(*fp).cp.is_null() && (OptDisplay == 5 as libc::c_int && check_important_feature(fp) != 0 || OptDisplay != 5 as libc::c_int && strncmp((*fp).cp, b"\xef\xbc\xb4\x00" as *const u8 as *const libc::c_char, strlen(b"\xef\xbc\xb4\x00" as *const u8 as *const libc::c_char)) != 0 || OptDisplay == 3 as libc::c_int) {
            print_one_feature((*fp).cp, filep);
        }
        fp = (*fp).next
    };
}

#[no_mangle]
pub unsafe extern "C" fn print_some_feature(mut fp: *mut FEATURE, mut filep: *mut FILE) {
    /* <f1><f2> ... <f3> という形式の出力 
       指定したものだけを表示 */
    while !fp.is_null() {
        if !(*fp).cp.is_null() && check_important_feature(fp) != 0 {
            print_one_feature((*fp).cp, filep);
        }
        fp = (*fp).next
    };
}

#[no_mangle]
pub unsafe extern "C" fn print_feature2(mut fp: *mut FEATURE, mut filep: *mut FILE) {
    /* (f1 f2 ... f3) という形式の出力
       (ただしＴではじまるfeatureは表示しない) */
    if !fp.is_null() {
        fprintf(filep, b"(\x00" as *const u8 as *const libc::c_char);
        while !fp.is_null() {
            if !(*fp).cp.is_null() && strncmp((*fp).cp, b"\xef\xbc\xb4\x00" as *const u8 as *const libc::c_char, strlen(b"\xef\xbc\xb4\x00" as *const u8 as *const libc::c_char)) != 0 {
                fprintf(filep, b"%s\x00" as *const u8 as *const libc::c_char, (*fp).cp);
                if !(*fp).next.is_null() {
                    fprintf(filep, b" \x00" as *const u8 as *const libc::c_char);
                }
            }
            fp = (*fp).next
        }
        fprintf(filep, b")\x00" as *const u8 as *const libc::c_char);
    } else {
        fprintf(filep, b"NIL\x00" as *const u8 as *const libc::c_char);
    };
}

#[no_mangle]
pub unsafe extern "C" fn clear_feature(mut fpp: *mut *mut FEATURE) {
    let mut fp: *mut FEATURE = 0 as *mut FEATURE;
    let mut next: *mut FEATURE = 0 as *mut FEATURE;
    fp = *fpp;
    *fpp = 0 as *mut FEATURE;
    while !fp.is_null() {
        next = (*fp).next;
        free((*fp).cp as *mut libc::c_void);
        free(fp as *mut libc::c_void);
        fp = next
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn delete_cfeature(mut fpp: *mut *mut FEATURE, mut type_0: *mut libc::c_char) {
    let mut prep: *mut FEATURE = 0 as *mut FEATURE;
    while !(*fpp).is_null() {
        if comp_feature((**fpp).cp, type_0) == (0 as libc::c_int == 0) as libc::c_int {
            let mut next: *mut FEATURE = 0 as *mut FEATURE;
            free((**fpp).cp as *mut libc::c_void);
            if prep.is_null() {
                next = (**fpp).next;
                free(*fpp as *mut libc::c_void);
                *fpp = next
            } else {
                next = (**fpp).next;
                free(*fpp as *mut libc::c_void);
                (*prep).next = next
            }
            return;
        }
        prep = *fpp;
        fpp = &mut (*prep).next
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn delete_alt_feature(mut fpp: *mut *mut FEATURE) {
    /* <ALT-...>を削除 */
    let mut prep: *mut FEATURE = 0 as *mut FEATURE;
    while !(*fpp).is_null() {
        if strncmp((**fpp).cp, b"ALT-\x00" as *const u8 as *const libc::c_char, 4 as libc::c_int as libc::c_ulong) == 0 {
            let mut next: *mut FEATURE = 0 as *mut FEATURE;
            free((**fpp).cp as *mut libc::c_void);
            if prep.is_null() {
                next = (**fpp).next;
                free(*fpp as *mut libc::c_void);
                *fpp = next
                /* prepはNULLのまま */
            } else {
                /* prepがあるとき */
                next = (**fpp).next; /* prepは現状維持 */
                free(*fpp as *mut libc::c_void);
                (*prep).next = next;
                fpp = &mut (*prep).next
            }
        } else {
            prep = *fpp;
            fpp = &mut (*prep).next
        }
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn delete_cfeature_from_mrphs(mut m_ptr: *mut MRPH_DATA, mut length: libc::c_int, mut type_0: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length {
        delete_cfeature(&mut (*m_ptr.offset(i as isize)).f, type_0);
        i += 1
    };
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn delete_temp_feature(mut fpp: *mut *mut FEATURE) {
    /* 仮付与したfeatureを削除 */
    let mut prep: *mut FEATURE = 0 as *mut FEATURE;
    while !(*fpp).is_null() {
        if comp_feature((**fpp).cp, b"\xe4\xbb\xae\xe4\xbb\x98\xe4\xb8\x8e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char) == (0 as libc::c_int == 0) as libc::c_int {
            let mut next: *mut FEATURE = 0 as *mut FEATURE;
            free((**fpp).cp as *mut libc::c_void);
            if prep.is_null() {
                next = (**fpp).next;
                free(*fpp as *mut libc::c_void);
                *fpp = next
            } else {
                next = (**fpp).next;
                free(*fpp as *mut libc::c_void);
                (*prep).next = next
            }
            fpp = &mut (*prep).next
        } else {
            prep = *fpp;
            fpp = &mut (*prep).next
        }
    };
}
/*
 *
 *  ファイル(S式または文字列) ==コピー==> ルール構造体
 *
 */
#[no_mangle]
pub unsafe extern "C" fn copy_cfeature(mut fpp: *mut *mut FEATURE, mut fname: *mut libc::c_char) {
    while !(*fpp).is_null() { fpp = &mut (**fpp).next }
    *fpp = malloc(::std::mem::size_of::<FEATURE>() as libc::c_ulong) as *mut FEATURE;
    if (*fpp).is_null() || {
        (**fpp).cp = malloc(strlen(fname).wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
        (**fpp).cp.is_null()
    } {
        fprintf(stderr, b"Can\'t allocate memory for FEATURE\n\x00" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    strcpy((**fpp).cp, fname);
    (**fpp).next = 0 as FEATUREptr;
}

#[no_mangle]
pub unsafe extern "C" fn list2feature(mut cp: *mut CELL, mut fpp: *mut *mut FEATURE) {
    while !car(cp).is_null() {
        copy_cfeature(fpp, (*car(cp)).value.atom as *mut libc::c_char);
        fpp = &mut (**fpp).next;
        cp = cdr(cp)
    };
}

#[no_mangle]
pub unsafe extern "C" fn list2feature_pattern(mut f: *mut FEATURE_PATTERN, mut cell: *mut CELL) {
    /* リスト ((文頭)(体言)(提題)) などをFEATURE_PATTERNに変換 */
    let mut nth: libc::c_int = 0 as libc::c_int; /* ?? &(f->fp[nth]) */
    while !car(cell).is_null() {
        clear_feature((*f).fp.as_mut_ptr().offset(nth as
            isize)); /* ?? &(f->fp[nth]) */
        list2feature(car(cell), (*f).fp.as_mut_ptr().offset(nth as isize));
        cell = cdr(cell);
        nth += 1
    }
    (*f).fp[nth as usize] = 0 as *mut FEATURE;
}

#[no_mangle]
pub unsafe extern "C" fn string2feature_pattern_OLD(mut f: *mut FEATURE_PATTERN, mut cp: *mut libc::c_char) {
    /* 文字列 "文頭|体言|提題" などをFEATURE_PATTERNに変換
       本来list2feature_patternに対応するものだが,
       ORだけでANDはサポートしていない */
    let mut nth: libc::c_int = 0 as libc::c_int; /* ?? &(f->fp[nth]) */
    let mut scp: *mut libc::c_char = 0 as *mut libc::c_char; /* ?? &(f->fp[nth]) */
    let mut ecp: *mut libc::c_char = 0 as *mut libc::c_char; /* ?? &(f->fp[nth]) */
    if cp.is_null() || *cp.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
        (*f).fp[nth as usize] = 0 as *mut FEATURE;
        return;
    }
    strcpy(feature_buffer.as_mut_ptr(), cp);
    ecp = feature_buffer.as_mut_ptr();
    scp = ecp;
    while *ecp != 0 {
        if *ecp as libc::c_int == '|' as i32 {
            *ecp = '\u{0}' as i32 as libc::c_char;
            clear_feature((*f).fp.as_mut_ptr().offset(nth as isize));
            copy_cfeature((*f).fp.as_mut_ptr().offset(nth as isize), scp);
            nth += 1;
            scp = ecp.offset(1 as libc::c_int as isize)
        }
        ecp = ecp.offset(1)
    }
    clear_feature((*f).fp.as_mut_ptr().offset(nth as isize));
    copy_cfeature(&mut *(*f).fp.as_mut_ptr().offset(nth as isize), scp);
    nth += 1;
    (*f).fp[nth as usize] = 0 as *mut FEATURE;
}

#[no_mangle]
pub unsafe extern "C" fn string2feature_pattern(mut f: *mut FEATURE_PATTERN, mut cp: *mut libc::c_char) {
    /* 文字列 "文頭|体言|提題" などをFEATURE_PATTERNに変換
       本来list2feature_patternに対応するものだが,
       ORだけでANDはサポートしていない */
    let mut nth: libc::c_int = 0;
    let mut start_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut loop_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fpp: *mut *mut FEATURE = 0 as *mut *mut FEATURE;
    if *cp == 0 {
        (*f).fp[0 as libc::c_int as usize] = 0 as *mut FEATURE;
        return;
    }
    strcpy(feature_buffer.as_mut_ptr(), cp);
    nth = 0 as libc::c_int;
    clear_feature((*f).fp.as_mut_ptr().offset(nth as isize));
    fpp = (*f).fp.as_mut_ptr().offset(nth as isize);
    loop_cp = feature_buffer.as_mut_ptr();
    start_cp = loop_cp;
    while *loop_cp != 0 {
        if *loop_cp as libc::c_int == '&' as i32 && *loop_cp.offset(1 as libc::c_int as isize) as libc::c_int == '&' as i32 {
            *loop_cp = '\u{0}' as i32 as libc::c_char;
            copy_cfeature(fpp, start_cp);
            fpp = &mut (**fpp).next;
            loop_cp = loop_cp.offset(2 as libc::c_int as isize);
            start_cp = loop_cp
        } else if *loop_cp as libc::c_int == '|' as i32 && *loop_cp.offset(1 as libc::c_int as isize) as libc::c_int == '|' as i32 {
            *loop_cp = '\u{0}' as i32 as libc::c_char;
            copy_cfeature(fpp, start_cp);
            nth += 1;
            clear_feature((*f).fp.as_mut_ptr().offset(nth as isize));
            fpp = (*f).fp.as_mut_ptr().offset(nth as isize);
            loop_cp = loop_cp.offset(2 as libc::c_int as isize);
            start_cp = loop_cp
        } else {
            loop_cp = loop_cp.offset(1)
        }
    }
    copy_cfeature(fpp, start_cp);
    nth += 1;
    (*f).fp[nth as usize] = 0 as *mut FEATURE;
}
/*
 *
 * ルール構造体 ==付与==> 形態素または文節構造体
 *
 */
#[no_mangle]
pub unsafe extern "C" fn append_feature(mut fpp: *mut *mut FEATURE, mut afp: *mut FEATURE) {
    while !(*fpp).is_null() {
        fpp = &mut (**fpp).next
    }
    *fpp = afp;
}

#[no_mangle]
pub unsafe extern "C" fn assign_cfeature(mut fpp: *mut *mut FEATURE, mut fname: *mut libc::c_char, mut temp_assign_flag: libc::c_int) {
    /* temp_assign_flag: TRUEのとき「仮付与」を頭につける */
    /* 上書きの可能性をチェック */
    sscanf(fname, b"%[^:]\x00" as *const u8 as *const libc::c_char, feature_buffer.as_mut_ptr()); /* ※ fnameに":"がない場合はfeature_bufferはfname全体になる */
    /* quote('"')中の":"で切っていれば、もとに戻す */
    if strcmp(feature_buffer.as_mut_ptr(), fname) != 0 {
        let mut i: libc::c_int = 0;
        let mut count: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < strlen(feature_buffer.as_mut_ptr()) {
            if feature_buffer[i as usize] as libc::c_int == '\"' as i32 {
                count += 1
            }
            i += 1
        }
        if count % 2 as libc::c_int == 1 as libc::c_int {
            /* '"'が奇数 */
            strcpy(feature_buffer.as_mut_ptr(), fname);
        }
    }
    while !(*fpp).is_null() {
        if comp_feature((**fpp).cp, feature_buffer.as_mut_ptr()) == (0 as libc::c_int == 0) as libc::c_int {
            free((**fpp).cp as *mut libc::c_void);
            (**fpp).cp = malloc(strlen(fname).wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
            if (**fpp).cp.is_null() {
                fprintf(stderr, b"Can\'t allocate memory for FEATURE\n\x00" as *const u8 as *const libc::c_char);
                exit(-(1 as libc::c_int));
            }
            strcpy((**fpp).cp, fname);
            return;
            /* 上書きで終了 */
        }
        fpp = &mut (**fpp).next
    }
    /* 上書きできなければ末尾に追加 */
    *fpp = malloc(::std::mem::size_of::<FEATURE>() as libc::c_ulong) as *mut FEATURE;
    if (*fpp).is_null() ||
        {
            (**fpp).cp = malloc(strlen(fname).wrapping_add(strlen(b"\xe4\xbb\xae\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char)).wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
            (**fpp).cp.is_null()
        } {
        fprintf(stderr, b"Can\'t allocate memory for FEATURE\n\x00" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    if temp_assign_flag != 0 {
        strcpy((**fpp).cp, b"\xe4\xbb\xae\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char);
        strcat((**fpp).cp, fname);
    } else {
        strcpy((**fpp).cp, fname);
    }
    (**fpp).next = 0 as FEATUREptr;
}

#[no_mangle]
pub unsafe extern "C" fn str_delete_last_column(mut str: *mut libc::c_char) -> *mut libc::c_char {
    /* ':'区切りとみなし、2つ目以降のカラムを削除
       例: Wikipedia上位語:企業/きぎょう:0-3 → Wikipedia上位語:企業/きぎょう */
    return if !str.is_null() {
        let mut count: libc::c_int = 0 as libc::c_int;
        let mut ret: *mut libc::c_char = strdup(str);
        let mut cp: *mut libc::c_char = ret;
        loop {
            cp = strchr(cp, ':' as i32);
            if cp.is_null() { break; }
            /* 前から2つ目の':'を探す */
            if count == 1 as libc::c_int {
                /* 2つ目の':' */
                *cp = '\u{0}' as i32 as libc::c_char; /* あれば終端 */
                return ret;
            } /* 次のstrchrのために一つ進める */
            cp = cp.offset(1);
            count += 1
        }
        ret
    } else {
        0 as *mut libc::c_char
    };
}

#[no_mangle]
pub unsafe extern "C" fn assign_feature(mut fpp1: *mut *mut FEATURE, mut fpp2: *mut *mut FEATURE, mut ptr: *mut libc::c_void, mut offset: libc::c_int, mut length: libc::c_int, mut temp_assign_flag: libc::c_int) {
    /*
     *  ルールを適用の結果，ルールから構造体にFEATUREを付与する
     *  構造体自身に対する処理も可能としておく
     */
    let mut i: libc::c_int = 0;
    let mut assign_pos: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer: [libc::c_char; 5120] = [0; 5120];
    let mut fpp: *mut *mut FEATURE = 0 as *mut *mut FEATURE;
    let mut next: *mut FEATURE = 0 as *mut FEATURE;
    while !(*fpp2).is_null() {
        if *(**fpp2).cp as libc::c_int == '^' as i32 {
            /* 削除の場合 */
            fpp = fpp1; /* 追加の場合 */
            while !(*fpp).is_null() {
                if comp_feature((**fpp).cp,
                                &mut *(**fpp2).cp.offset(1 as libc::c_int as
                                    isize)) ==
                    (0 as libc::c_int == 0) as libc::c_int {
                    free((**fpp).cp as *mut libc::c_void);
                    next = (**fpp).next;
                    free(*fpp as *mut libc::c_void);
                    *fpp = next
                } else { fpp = &mut (**fpp).next }
            }
        } else if *(**fpp2).cp as libc::c_int == '&' as i32 {
            /* 関数の場合 */
            if strcmp((**fpp2).cp, b"&\xe8\xa1\xa8\xe5\xb1\xa4:\xe4\xbb\x98\xe4\xb8\x8e\x00" as *const u8 as *const libc::c_char) == 0 {
                set_pred_voice((ptr as *mut BNST_DATA).offset(offset as isize));
                get_scase_code((ptr as *mut BNST_DATA).offset(offset as isize)); /* ヴォイス */
                /* 表層格 */
            } else if strcmp((**fpp2).cp, b"&\xe8\xa1\xa8\xe5\xb1\xa4:\xe5\x89\x8a\xe9\x99\xa4\x00"
                as *const u8 as *const libc::c_char) == 0 {
                i = 0 as libc::c_int;
                cp = (*(ptr as *mut BNST_DATA).offset(offset as isize)).SCASE_code.as_mut_ptr();
                while i < 11 as libc::c_int {
                    *cp = 0 as libc::c_int as libc::c_char;
                    i += 1;
                    cp = cp.offset(1)
                }
            } else if strncmp((**fpp2).cp, b"&\xe8\xa1\xa8\xe5\xb1\xa4:^\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe8\xa1\xa8\xe5\xb1\xa4:^\x00" as *const u8 as *const libc::c_char)) == 0 {
                (*(ptr as *mut BNST_DATA).offset(offset as isize)).SCASE_code[case2num((**fpp2).cp.offset(strlen(b"&\xe8\xa1\xa8\xe5\xb1\xa4:^\x00" as *const u8 as *const libc::c_char) as isize)) as usize] = 0 as libc::c_int as libc::c_char
            } else if strncmp((**fpp2).cp, b"&\xe8\xa1\xa8\xe5\xb1\xa4:\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe8\xa1\xa8\xe5\xb1\xa4:\x00" as *const u8 as *const libc::c_char)) == 0 {
                (*(ptr as *mut BNST_DATA).offset(offset as isize)).SCASE_code[case2num((**fpp2).cp.offset(strlen(b"&\xe8\xa1\xa8\xe5\xb1\xa4:\x00" as *const u8 as *const libc::c_char) as isize)) as usize] = 1 as libc::c_int as libc::c_char
            } else if strncmp((**fpp2).cp, b"&MEMO:\x00" as *const u8 as *const libc::c_char, strlen(b"&MEMO:\x00" as *const u8 as *const libc::c_char)) == 0 {
                strcat(PM_Memo.as_mut_ptr(), b" \x00" as *const u8 as *const libc::c_char);
                strcat(PM_Memo.as_mut_ptr(), (**fpp2).cp.offset(strlen(b"&MEMO:\x00" as *const u8 as *const libc::c_char) as isize));
            } else if strncmp((**fpp2).cp, b"&\xe5\x93\x81\xe8\xa9\x9e\xe5\xa4\x89\xe6\x9b\xb4:\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe5\x93\x81\xe8\xa9\x9e\xe5\xa4\x89\xe6\x9b\xb4:\x00" as *const u8 as *const libc::c_char)) == 0 {
                if OptPosModification != 0 && OptInput & 1 as libc::c_int == 0 {
                    /* 解析済みではない場合に品詞変更を実行 */
                    change_mrph((ptr as *mut MRPH_DATA).offset(offset as isize), *fpp2);
                }
            } else if strncmp((**fpp2).cp, b"&\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4:\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4:\x00" as *const u8 as *const libc::c_char)) == 0 {
                change_one_mrph_rep((ptr as *mut MRPH_DATA).offset(offset as isize), 1 as libc::c_int, *(**fpp2).cp.offset(strlen(b"&\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\xe5\xa4\x89\xe6\x9b\xb4:\x00" as *const u8 as *const libc::c_char) as isize));
            } else if strncmp((**fpp2).cp, b"&\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char)) == 0 {
                assign_sm((ptr as *mut BNST_DATA).offset(offset as isize), (**fpp2).cp.offset(strlen(b"&\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char) as isize));
            } else if strncmp((**fpp2).cp, b"&\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9e\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9e\xe6\xa0\xbc\xe8\xa7\xa3\xe6\x9e\x90\x00" as *const u8 as *const libc::c_char)) == 0 {
                cp = make_fukugoji_case_string((ptr as *mut TAG_DATA).offset(offset as isize).offset(1 as libc::c_int as isize));
                if !cp.is_null() {
                    assign_cfeature(&mut (*(ptr as *mut TAG_DATA).offset(offset as isize)).f, cp, temp_assign_flag);
                }
            } else if strncmp((**fpp2).cp, b"&\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9eID\xe4\xbb\x98\xe4\xb8\x8e\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe8\xa4\x87\xe5\x90\x88\xe8\xbe\x9eID\xe4\xbb\x98\xe4\xb8\x8e\x00" as *const u8 as *const libc::c_char)) == 0 {
                cp = make_fukugoji_id((ptr as *mut BNST_DATA).offset(offset as isize));
                if !cp.is_null() {
                    assign_cfeature(&mut (*(ptr as *mut BNST_DATA).offset(offset as isize)).f, cp, temp_assign_flag);
                }
            } else if strncmp((**fpp2).cp, b"&\xe8\xa8\x98\xe6\x86\xb6\xe8\xaa\x9e\xe5\xbd\x99\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe8\xa8\x98\xe6\x86\xb6\xe8\xaa\x9e\xe5\xbd\x99\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char)) == 0 {
                sprintf(buffer.as_mut_ptr(), b"%s:%s\x00" as *const u8 as *const libc::c_char, (**fpp2).cp.offset(strlen(b"&\xe8\xa8\x98\xe6\x86\xb6\xe8\xaa\x9e\xe5\xbd\x99\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char) as isize), (*(matched_ptr as *mut MRPH_DATA)).Goi.as_mut_ptr());
                assign_cfeature(&mut (*(ptr as *mut BNST_DATA).offset(offset as isize)).f, buffer.as_mut_ptr(), temp_assign_flag);
            } else if strncmp((**fpp2).cp, b"&\xe8\xa8\x98\xe6\x86\xb6FEATURE\xe6\x98\x87\xe6\xa0\xbc:\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe8\xa8\x98\xe6\x86\xb6FEATURE\xe6\x98\x87\xe6\xa0\xbc:\x00" as *const u8 as *const libc::c_char)) == 0 {
                cp = check_feature((*(matched_ptr as *mut MRPH_DATA)).f, (**fpp2).cp.offset(strlen(b"&\xe8\xa8\x98\xe6\x86\xb6FEATURE\xe4\xbb\x98\xe4\xb8\x8e:\x00" as *const u8 as *const libc::c_char) as isize));
                if !cp.is_null() {
                    cp2 = str_delete_last_column(cp);
                    if !cp2.is_null() {
                        /* &記憶FEATURE昇格 : 記憶した形態素からFEATUREを探し、それを基本句に付与 */
                        /* ':'区切りの最後のカラムを削除 */
                        assign_cfeature(&mut (*(ptr as *mut TAG_DATA).offset(offset as isize)).f, cp2, temp_assign_flag);
                        free(cp2 as *mut libc::c_void);
                    }
                }
            } else if strncmp((**fpp2).cp, b"&\xe4\xbc\x9d\xe6\x90\xac:\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe4\xbc\x9d\xe6\x90\xac:\x00" as *const u8 as *const libc::c_char)) == 0 {
                pat = (**fpp2).cp.offset(strlen(b"&\xe4\xbc\x9d\xe6\x90\xac:\x00" as *const u8 as *const libc::c_char) as isize);
                sscanf(pat, b"%d\x00" as *const u8 as *const libc::c_char, &mut i as *mut libc::c_int);
                pat = strchr(pat, ':' as i32);
                pat = pat.offset(1);
                cp = check_feature((*(ptr as *mut TAG_DATA).offset(offset as isize)).f, pat);
                if !cp.is_null() {
                    assign_cfeature(&mut (*(ptr as *mut TAG_DATA).offset(offset as isize).offset(i as isize)).f, cp, temp_assign_flag);
                } else {
                    /* &伝搬:n:FEATURE : FEATUREの伝搬  */
                    /* ないなら、もとからあるものを削除 */
                    delete_cfeature(&mut (*(ptr as *mut TAG_DATA).offset(offset as isize).offset(i as isize)).f, pat);
                }
                if (*(ptr as *mut TAG_DATA).offset(offset as isize)).bnum >= 0 as libc::c_int {
                    /* 文節区切りでもあるとき */
                    cp = check_feature((*(*(ptr as *mut TAG_DATA).offset(offset as isize)).b_ptr).f, pat);
                    if !cp.is_null() {
                        assign_cfeature(&mut (*(*(ptr as *mut TAG_DATA).offset(offset as isize)).b_ptr.offset(i as isize)).f, cp, temp_assign_flag);
                    } else {
                        delete_cfeature(&mut (*(*(ptr as *mut TAG_DATA).offset(offset as isize)).b_ptr.offset(i as isize)).f, pat);
                    }
                }
            } else if strncmp((**fpp2).cp, b"&\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe4\xbb\x98\xe5\xb1\x9e\xe5\x8c\x96\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe4\xbb\x98\xe5\xb1\x9e\xe5\x8c\x96\x00" as *const u8 as *const libc::c_char)) == 0 {
                i = 0 as libc::c_int;
                while i < (*(ptr as *mut TAG_DATA).offset(offset as isize)).mrph_num {
                    delete_cfeature(&mut (*(*(ptr as *mut TAG_DATA).offset(offset as isize)).mrph_ptr.offset(i as isize)).f, b"\xe8\x87\xaa\xe7\xab\x8b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                    delete_cfeature(&mut (*(*(ptr as *mut TAG_DATA).offset(offset as isize)).mrph_ptr.offset(i as isize)).f, b"\xe5\x86\x85\xe5\xae\xb9\xe8\xaa\x9e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                    delete_cfeature(&mut (*(*(ptr as *mut TAG_DATA).offset(offset as isize)).mrph_ptr.offset(i as isize)).f, b"\xe6\xba\x96\xe5\x86\x85\xe5\xae\xb9\xe8\xaa\x9e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                    assign_cfeature(&mut (*(*(ptr as *mut TAG_DATA).offset(offset as isize)).mrph_ptr.offset(i as isize)).f, b"\xe4\xbb\x98\xe5\xb1\x9e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char, temp_assign_flag);
                    i += 1
                }
            } else if strncmp((**fpp2).cp, b"&\xe8\x87\xaa\xe5\x8b\x95\xe8\xbe\x9e\xe6\x9b\xb8:\x00" as *const u8 as *const libc::c_char, strlen(b"&\xe8\x87\xaa\xe5\x8b\x95\xe8\xbe\x9e\xe6\x9b\xb8:\x00" as *const u8 as *const libc::c_char)) == 0 {
                if offset == 0 as libc::c_int {
                    if strncmp((**fpp2).cp.offset(strlen(b"&\xe8\x87\xaa\xe5\x8b\x95\xe8\xbe\x9e\xe6\x9b\xb8:\x00" as *const u8 as *const libc::c_char) as isize),
                               b"\xe5\x85\x88\xe9\xa0\xad:\x00" as *const u8 as *const libc::c_char,
                               strlen(b"\xe5\x85\x88\xe9\xa0\xad:\x00" as *const u8 as *const libc::c_char))
                        == 0 {
                        assign_pos = 0 as libc::c_int
                    } else if strncmp((**fpp2).cp.offset(strlen(b"&\xe8\x87\xaa\xe5\x8b\x95\xe8\xbe\x9e\xe6\x9b\xb8:\x00" as *const u8 as *const libc::c_char) as isize), b"\xe6\x9c\xab\xe5\xb0\xbe:\x00" as *const u8 as *const libc::c_char, strlen(b"\xe6\x9c\xab\xe5\xb0\xbe:\x00" as *const u8 as *const libc::c_char)) == 0 {
                        assign_pos = length - 1 as libc::c_int
                    } else {
                        fprintf(stderr, b";; Invalid feature: %s\n\x00" as *const u8 as *const libc::c_char, (**fpp2).cp);
                        exit(-(1 as libc::c_int));
                    }
                    check_auto_dic(ptr as *mut MRPH_DATA, assign_pos, length, (**fpp2).cp.offset(strlen(b"&\xe8\x87\xaa\xe5\x8b\x95\xe8\xbe\x9e\xe6\x9b\xb8:\xe5\x85\x88\xe9\xa0\xad:\x00" as *const u8 as *const libc::c_char) as isize), temp_assign_flag);
                }
            }
        } else {
            assign_cfeature(fpp1, (**fpp2).cp, temp_assign_flag);
        }
        fpp2 = &mut (**fpp2).next
    };
}

/// 形態素付属化 : 属する形態素列をすべて<付属>にする
///    本来は、&形態素feature:^自立 のように引き数をとるべき */
/// 自動辞書 : 自動獲得した辞書をチェック (マッチ部分全体) */
#[no_mangle]
pub unsafe extern "C" fn copy_feature(mut dst_fpp: *mut *mut FEATURE, mut src_fp: *mut FEATURE) {
    while !src_fp.is_null() {
        assign_cfeature(dst_fpp, (*src_fp).cp, 0 as libc::c_int);
        src_fp = (*src_fp).next
    };
}


/// ルール構造体 <==照合==> 形態素または文節構造体
/// 完全一致 または 部分一致(patternが短く,次の文字が':')ならマッチ
#[no_mangle]
pub unsafe extern "C" fn comp_feature(mut data: *mut libc::c_char, mut pattern: *mut libc::c_char) -> libc::c_int {
    return if !data.is_null() && strcmp(data, pattern) == 0 {
        (0 as libc::c_int == 0) as libc::c_int
    } else if !data.is_null() && strncmp(data, pattern, strlen(pattern)) == 0
        &&
        *data.offset(strlen(pattern) as isize) as libc::c_int ==
            ':' as i32 {
        (0 as libc::c_int == 0) as libc::c_int
    } else { 0 as libc::c_int };
}

#[no_mangle]
pub unsafe extern "C" fn comp_feature_NE(mut data: *mut libc::c_char, mut pattern: *mut libc::c_char) -> libc::c_int {
    let mut decision: [libc::c_char; 9] = [0; 9];
    decision[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    sscanf(data, b"%*[^:]:%*[^:]:%s\x00" as *const u8 as *const libc::c_char,
           decision.as_mut_ptr());
    return if decision[0 as libc::c_int as usize] as libc::c_int != 0 &&
        strcmp(decision.as_mut_ptr(), pattern) == 0 {
        (0 as libc::c_int == 0) as libc::c_int
    } else { 0 as libc::c_int };
}

#[no_mangle]
pub unsafe extern "C" fn check_feature(mut fp: *mut FEATURE, mut fname: *mut libc::c_char) -> *mut libc::c_char {
    while !fp.is_null() {
        if comp_feature((*fp).cp, fname) ==
            (0 as libc::c_int == 0) as libc::c_int {
            return (*fp).cp;
        }
        fp = (*fp).next
    }
    return 0 as *mut libc::c_char;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn check_feature_NE(mut fp: *mut FEATURE, mut fname: *mut libc::c_char) -> *mut libc::c_char {
    while !fp.is_null() {
        if comp_feature_NE((*fp).cp, fname) ==
            (0 as libc::c_int == 0) as libc::c_int {
            return (*fp).cp;
        }
        fp = (*fp).next
    }
    return 0 as *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn check_category(mut fp: *mut FEATURE, mut fname: *mut libc::c_char, mut strict_flag: libc::c_int) -> libc::c_int {
    /* strict_flag == TRUE: カテゴリが複数ある(曖昧な)ときはFALSEを返す */
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 as libc::c_int != 0 &&
        strlen(fname) == 1 as libc::c_int as libc::c_ulong {
        /* fnameが'a'または'v'の場合 */
        /* <代表表記:...[av]>もカテゴリの一種として扱う */
        if check_feature(fp, b"\xe7\x96\x91\xe4\xbc\xbc\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00" as *const u8 as *const libc::c_char as *mut libc::c_char).is_null() && {
            cp = check_feature(fp, b"\xe4\xbb\xa3\xe8\xa1\xa8\xe8\xa1\xa8\xe8\xa8\x98\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
            !cp.is_null()
        } && *cp.offset(strlen(cp) as isize).offset(-(1 as libc::c_int as isize)) as libc::c_int == *fname as libc::c_int {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
    } else {
        cp =
            check_feature(fp, b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if !cp.is_null() {
            if strict_flag != 0 && !strchr(cp, ';' as i32).is_null() {
                /* strict_flag時: カテゴリが複数あるときはFALSE */
                return 0 as libc::c_int;
            }
            /* 複数ある場合は";"で区切られている */
            cp = cp.offset(strlen(b"\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa\x00" as *const u8 as *const libc::c_char) as isize); /* ":"の分はdoの中で足す */
            loop {
                cp = cp.offset(1); /* ":"もしくは";"の分 */
                if strcmp(cp, fname) == 0 || strncmp(cp, fname, strlen(fname)) == 0 && *cp.offset(strlen(fname) as isize) as libc::c_int == ';' as i32 {
                    return (0 as libc::c_int == 0) as libc::c_int;
                }
                cp = strchr(cp, ';' as i32);
                if cp.is_null() { break; }
            }
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn compare_threshold(mut value: libc::c_int, mut threshold: libc::c_int, mut eq: *mut libc::c_char) -> libc::c_int {
    if strcmp(eq, b"lt\x00" as *const u8 as *const libc::c_char) == 0 {
        return if value < threshold {
            (0 as libc::c_int == 0) as libc::c_int
        } else {
            0 as libc::c_int
        };
    } else {
        if strcmp(eq, b"le\x00" as *const u8 as *const libc::c_char) == 0 {
            return if value <= threshold {
                (0 as libc::c_int == 0) as libc::c_int
            } else {
                0 as libc::c_int
            };
        } else {
            if strcmp(eq, b"gt\x00" as *const u8 as *const libc::c_char) == 0 {
                return if value > threshold {
                    (0 as libc::c_int == 0) as libc::c_int
                } else {
                    0 as libc::c_int
                };
            } else {
                if strcmp(eq, b"ge\x00" as *const u8 as *const libc::c_char)
                    == 0 {
                    return if value >= threshold {
                        (0 as libc::c_int == 0) as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                }
            }
        }
    }
    return 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn check_Bunrui(mut mp: *mut MRPH_DATA, mut class: *mut libc::c_char, mut flag: libc::c_int) -> libc::c_int {
    let mut string: [libc::c_char; 14] = [0; 14];
    if strcmp(Class[6 as libc::c_int as usize][(*mp).Bunrui as usize].id as *const libc::c_char, class) == 0 {
        return flag;
    }
    sprintf(string.as_mut_ptr(), b"\xe5\x93\x81\xe6\x9b\x96-%s\x00" as *const u8 as *const libc::c_char, class);
    if !check_feature((*mp).f, string.as_mut_ptr()).is_null() {
        return flag;
    }
    return 1 as libc::c_int - flag;
}

#[no_mangle]
pub unsafe extern "C" fn check_char_type(mut code: libc::c_int) -> libc::c_int {
    /* for Unicode */
    /* HIRAGANA */
    return if code > 0x303f as libc::c_int && code < 0x30a0 as libc::c_int {
        2 as libc::c_int
    } else if code > 0x309f as libc::c_int && code < 0x30fb as libc::c_int || code == 0x30fc as libc::c_int {
        1 as libc::c_int
    } else if code > 0x2fff as libc::c_int && code < 0x3003 as libc::c_int || code == 0xff0c as libc::c_int || code == 0xff0e as libc::c_int {
        64 as libc::c_int
    } else if code > 0xff0f as libc::c_int && code < 0xff1a as libc::c_int {
        8 as libc::c_int
    } else if code > 0xff20 as libc::c_int && code < 0xff3b as libc::c_int || code > 0xff40 as libc::c_int && code < 0xff5b as libc::c_int {
        16 as libc::c_int
    } else if code > 0x4dff as libc::c_int && code < 0xa000 as libc::c_int || code == 0x3005 as libc::c_int {
        4 as libc::c_int
    } else {
        32 as libc::c_int
    };
}

/// KATAKANA and "ー"(0x30fc) */
/// PUNCTUATIONS (　、。，．) */
/// FIGURE (only ０-９) */
/// ALPHABET (Ａ-Ｚ, ａ-ｚ) */
/// CJK Unified Ideographs and "々" */
#[no_mangle]
pub unsafe extern "C" fn check_str_type(mut ucp: *mut libc::c_uchar, mut allowed_type: libc::c_int, mut length: libc::c_int) -> libc::c_int {
    let mut code: libc::c_int = 0 as libc::c_int;
    let mut precode: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut unicode: libc::c_int = 0;
    if length == 0 as libc::c_int {
        length = strlen(ucp as *const libc::c_char) as libc::c_int
    }
    while i < length {
        let mut c: libc::c_uchar = *ucp.offset(i as isize);
        if c as libc::c_int > 0xfb as libc::c_int {
            // 6 bytes
            code = 0 as libc::c_int;
            i += 6 as libc::c_int
        } else if c as libc::c_int > 0xf7 as libc::c_int {
            // 5 bytes
            code = 0 as libc::c_int;
            i += 5 as libc::c_int
        } else if c as libc::c_int > 0xef as libc::c_int {
            // 4 bytes
            code = 0 as libc::c_int;
            i += 4 as libc::c_int
        } else if c as libc::c_int > 0xdf as libc::c_int {
            // 3 bytes
            unicode =
                (c as libc::c_int & 0xf as libc::c_int) << 12 as libc::c_int;
            c = *ucp.offset(i as isize).offset(1 as libc::c_int as isize);
            unicode +=
                (c as libc::c_int & 0x3f as libc::c_int) << 6 as libc::c_int;
            c = *ucp.offset(i as isize).offset(2 as libc::c_int as isize);
            unicode += c as libc::c_int & 0x3f as libc::c_int;
            code = check_char_type(unicode);
            i += 3 as libc::c_int
        } else if c as libc::c_int > 0x7f as libc::c_int {
            // 2 bytes
            unicode =
                (c as libc::c_int & 0x1f as libc::c_int) << 6 as libc::c_int;
            c = *ucp.offset(i as isize).offset(1 as libc::c_int as isize);
            unicode += c as libc::c_int & 0x3f as libc::c_int;
            code = check_char_type(unicode);
            i += 2 as libc::c_int
        } else {
            // 1 byte
            code = check_char_type(c as libc::c_int);
            i += 1
        }
        if allowed_type != 0 {
            if code & allowed_type == 0 { return 0 as libc::c_int; }
        } else if precode != 0 && precode != code {
            return 0 as libc::c_int;
            /* code is mixed */
        }
        precode = code
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}

/// rule : ルール
/// fd : データ側のFEATURE
/// p1 : 係り受けの場合，係り側の構造体(MRPH_DATA,BNST_DATAなど)
/// p2 : データの構造体(MRPH_DATA,BNST_DATAなど)
#[no_mangle]
pub unsafe extern "C" fn check_function(mut rule: *mut libc::c_char, mut fd: *mut FEATURE, mut ptr1: *mut libc::c_void, mut ptr2: *mut libc::c_void) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    // let mut type_0: libc::c_int = 0;
    // let mut pretype: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ucp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* &記英数カ : 記英数カ チェック (句読点以外) (形態素レベル) */
    return if strcmp(rule,
                     b"&\xe8\xa8\x98\xe8\x8b\xb1\xe6\x95\xb0\xe3\x82\xab\x00" as
                         *const u8 as *const libc::c_char) == 0 {
        check_str_type((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                           *mut libc::c_uchar,
                       32 as libc::c_int | 16 as libc::c_int |
                           8 as libc::c_int | 1 as libc::c_int,
                       0 as libc::c_int)
    } else if strcmp(rule,
                     b"&\xe6\xbc\xa2\xe5\xad\x97\x00" as *const u8 as
                         *const libc::c_char) == 0 {
        /* &漢字 : 漢字 チェック (形態素レベル) */
        /* euc-jp */
        /* 先頭の「か」「カ」「ヶ」は OK */
        if strncmp((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr(),
                   b"\xe3\x81\x8b\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_ulong) == 0 ||
            strncmp(
                (*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr(),
                b"\xe3\x82\xab\x00" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0 || strncmp(
            (*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr(),
            b"\xe3\x83\xb6\x00" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 {
            if strlen((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr()) == 3 as libc::c_int as libc::c_ulong {
                (0 as libc::c_int == 0) as libc::c_int;
            } else {
                check_str_type(
                    (*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut libc::c_uchar,
                    4 as libc::c_int,
                    0 as libc::c_int,
                );
            }
        } else {
            check_str_type((*(ptr2 as
                *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                               *mut libc::c_uchar, 4 as libc::c_int,
                           0 as libc::c_int)
        }
    } else if strcmp(rule,
                     b"&\xe3\x81\x8b\xe3\x81\xaa\xe6\xbc\xa2\xe5\xad\x97\x00"
                         as *const u8 as *const libc::c_char) == 0 {
        check_str_type((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                           *mut libc::c_uchar,
                       4 as libc::c_int | 2 as libc::c_int,
                       0 as libc::c_int)
    } else if strcmp(rule,
                     b"&\xe3\x81\xb2\xe3\x82\x89\xe3\x81\x8c\xe3\x81\xaa\x00"
                         as *const u8 as *const libc::c_char) == 0 {
        check_str_type((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                           *mut libc::c_uchar, 2 as libc::c_int,
                       0 as libc::c_int)
    } else if strcmp(rule,
                     b"&\xe6\x9c\xab\xe5\xb0\xbe\xe3\x81\xb2\xe3\x82\x89\xe3\x81\x8c\xe3\x81\xaa\x00"
                         as *const u8 as *const libc::c_char) == 0 {
        /* &かな漢字 : かな漢字チェック (形態素レベル) */
        /* &ひらがな : ひらがな チェック (形態素レベル) */
        /* &末尾ひらがな : 末尾の一文字がひらがなか チェック (形態素レベル) */
        ucp =
            (*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                *mut libc::c_uchar; /* 表記をチェック */
        ucp =
            ucp.offset(strlen(ucp as
                *const libc::c_char).wrapping_sub(3 as
                libc::c_int
                as
                libc::c_ulong)
                as isize);
        check_str_type(ucp, 2 as libc::c_int, 0 as libc::c_int)
    } else if strncmp(rule,
                      b"&\xe6\x9c\xab\xe5\xb0\xbe\xe6\x96\x87\xe5\xad\x97\xe5\x88\x97:\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe6\x9c\xab\xe5\xb0\xbe\xe6\x96\x87\xe5\xad\x97\xe5\x88\x97:\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe6\x9c\xab\xe5\xb0\xbe\xe6\x96\x87\xe5\xad\x97\xe5\x88\x97:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        /* &末尾文字列 : 末尾の文字列を チェック (形態素レベル) */
        /* パターンの方が大きければFALSE */
        if strlen(cp) > strlen((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr())
        {
            return 0 as libc::c_int;
        } /* 表記をチェック */
        ucp =
            (*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                *mut libc::c_uchar;
        ucp =
            ucp.offset(strlen(ucp as
                *const libc::c_char).wrapping_sub(strlen(cp))
                as isize);
        if strcmp(ucp as *const libc::c_char, cp) != 0 {
            return 0 as libc::c_int;
        }
        (0 as libc::c_int == 0) as libc::c_int
    } else if strcmp(rule,
                     b"&\xe3\x82\xab\xe3\x82\xbf\xe3\x82\xab\xe3\x83\x8a\x00"
                         as *const u8 as *const libc::c_char) == 0 {
        check_str_type((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                           *mut libc::c_uchar, 1 as libc::c_int,
                       0 as libc::c_int)
    } else if strcmp(rule,
                     b"&\xe6\x95\xb0\xe5\xad\x97\x00" as *const u8 as
                         *const libc::c_char) == 0 {
        check_str_type((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                           *mut libc::c_uchar, 8 as libc::c_int,
                       0 as libc::c_int)
    } else if strcmp(rule,
                     b"&\xe8\x8b\xb1\xe8\xa8\x98\xe5\x8f\xb7\x00" as *const u8
                         as *const libc::c_char) == 0 {
        check_str_type((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                           *mut libc::c_uchar,
                       16 as libc::c_int | 32 as libc::c_int |
                           64 as libc::c_int, 0 as libc::c_int)
    } else if strcmp(rule,
                     b"&\xe8\xa8\x98\xe5\x8f\xb7\x00" as *const u8 as
                         *const libc::c_char) == 0 {
        check_str_type((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                           *mut libc::c_uchar,
                       32 as libc::c_int | 64 as libc::c_int,
                       0 as libc::c_int)
    } else if strcmp(rule,
                     b"&\xe6\xb7\xb7\xe5\x90\x88\x00" as *const u8 as
                         *const libc::c_char) == 0 {
        if check_str_type((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                              *mut libc::c_uchar, 0 as libc::c_int,
                          0 as libc::c_int) == 0 as libc::c_int {
            /* &カタカナ : カタカナ チェック (形態素レベル) */
            /* &数字 : 数字 チェック (形態素レベル) */
            /* &英記号 : 英記号 チェック (形態素レベル) */
            /* &記号 : 記号 チェック (形態素レベル) */
            /* &混合 : 混合 (漢字+...) チェック (形態素レベル) */
            /* mixed */
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else if strcmp(rule,
                     b"&\xe4\xb8\x80\xe6\x96\x87\xe5\xad\x97\x00" as *const u8
                         as *const libc::c_char) == 0 {
        if strlen((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr()) <=
            3 as libc::c_int as libc::c_ulong {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else if strncmp(rule,
                      b"&\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa:\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa:\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe3\x82\xab\xe3\x83\x86\xe3\x82\xb4\xe3\x83\xaa:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        if strcmp(cp,
                  b"\xe6\x99\x82\xe9\x96\x93\x00" as *const u8 as
                      *const libc::c_char) == 0 {
            /* &一文字 : 文字数 チェック (形態素レベル) */
            /* &カテゴリ : カテゴリ チェック (形態素レベル) */
            /* 「時間」のときは、strict_flag == TRUE (曖昧なときはFALSEを返す) */
            check_category((*(ptr2 as *mut MRPH_DATA)).f, cp,
                           (0 as libc::c_int == 0) as libc::c_int)
        } else {
            check_category((*(ptr2 as *mut MRPH_DATA)).f, cp,
                           0 as libc::c_int)
        }
    } else if strncmp(rule,
                      b"&\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0:\x00" as
                          *const u8 as *const libc::c_char,
                      strlen(b"&\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0:\x00" as
                          *const u8 as *const libc::c_char)) == 0 {
        if Thesaurus != 2 as libc::c_int ||
            (*(ptr2 as *mut MRPH_DATA)).SM.is_null() {
            return 0 as libc::c_int;
        }
        cp =
            rule.offset(strlen(b"&\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        /* &意味素: 意味素チェック (形態素) */
        /* 漢字だったら意味属性名, それ以外ならコードそのまま */
        if *cp as libc::c_int & 0x80 as libc::c_int != 0 {
            if SM2CODEExist == (0 as libc::c_int == 0) as libc::c_int {
                cp = sm2code(cp)
            } else { cp = 0 as *mut libc::c_char }
            flag = 1 as libc::c_int
        } else { flag = 3 as libc::c_int }
        if !cp.is_null() {
            i = 0 as libc::c_int;
            while *(*(ptr2 as *mut MRPH_DATA)).SM.offset(i as isize) != 0 {
                if _sm_match_score(cp,
                                   &mut *(*(ptr2 as
                                       *mut MRPH_DATA)).SM.offset(i
                                       as
                                       isize),
                                   flag) != 0 {
                    return (0 as libc::c_int == 0) as libc::c_int;
                }
                i += 12 as libc::c_int
            }
        }
        0 as libc::c_int
    } else if strncmp(rule,
                      b"&\xe6\x96\x87\xe7\xaf\x80\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0:\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe6\x96\x87\xe7\xaf\x80\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0:\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        if Thesaurus != 2 as libc::c_int && Thesaurus != 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        cp =
            rule.offset(strlen(b"&\xe6\x96\x87\xe7\xaf\x80\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        /* &文節意味素: 意味素チェック (文節) */
        /* 漢字だったら意味属性名, それ以外ならコードそのまま */
        if *cp as libc::c_int & 0x80 as libc::c_int != 0 {
            if SM2CODEExist == (0 as libc::c_int == 0) as libc::c_int {
                cp = sm2code(cp)
            } else { cp = 0 as *mut libc::c_char }
            flag = 1 as libc::c_int
        } else { flag = 3 as libc::c_int }
        if !cp.is_null() {
            if Thesaurus == 2 as libc::c_int {
                if sm_match_check(cp,
                                  (*(ptr2 as
                                      *mut BNST_DATA)).SM_code.as_mut_ptr(),
                                  flag) != 0 {
                    return (0 as libc::c_int == 0) as libc::c_int;
                }
            } else if Thesaurus == 1 as libc::c_int {
                if bgh_match_check(cp,
                                   (*(ptr2 as
                                       *mut BNST_DATA)).BGH_code.as_mut_ptr())
                    != 0 {
                    return (0 as libc::c_int == 0) as libc::c_int;
                }
            }
        }
        0 as libc::c_int
    } else if strncmp(rule,
                      b"&\xe6\x96\x87\xe7\xaf\x80\xe5\x85\xa8\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0:\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe6\x96\x87\xe7\xaf\x80\xe5\x85\xa8\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0:\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        if Thesaurus != 2 as libc::c_int && Thesaurus != 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        cp =
            rule.offset(strlen(b"&\xe6\x96\x87\xe7\xaf\x80\xe5\x85\xa8\xe6\x84\x8f\xe5\x91\xb3\xe7\xb4\xa0:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        /* &文節全意味素: 文節のすべての意味素が指定意味素以下にあるかどうか */
        /* 漢字だったら意味属性名, それ以外ならコードそのまま */
        if *cp as libc::c_int & 0x80 as libc::c_int != 0 {
            if SM2CODEExist == (0 as libc::c_int == 0) as libc::c_int {
                cp = sm2code(cp)
            } else { cp = 0 as *mut libc::c_char }
        }
        if Thesaurus == 2 as libc::c_int {
            if !cp.is_null() &&
                (*(ptr2 as
                    *mut BNST_DATA)).SM_code[0 as libc::c_int as usize]
                    as libc::c_int != 0 &&
                sm_all_match((*(ptr2 as
                    *mut BNST_DATA)).SM_code.as_mut_ptr(),
                             cp) != 0 {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
        } else if Thesaurus == 1 as libc::c_int {
            if !cp.is_null() &&
                (*(ptr2 as
                    *mut BNST_DATA)).BGH_code[0 as libc::c_int as usize]
                    as libc::c_int != 0 &&
                sm_all_match((*(ptr2 as
                    *mut BNST_DATA)).BGH_code.as_mut_ptr(),
                             cp) != 0 {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
        }
        0 as libc::c_int
    } else if strncmp(rule,
                      b"&\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe9\x95\xb7:\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe9\x95\xb7:\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe9\x95\xb7:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        if *cp.offset(strlen(cp) as
            isize).offset(-(1 as libc::c_int as isize)) as
            libc::c_int == '-' as i32 {
            /* 形態素の長さ */
            /* 数字の後に"-"がついていれば */
            flag = 1 as libc::c_int; /* 指定長さ以上でOK */
            *cp.offset(strlen(cp) as
                isize).offset(-(1 as libc::c_int as isize)) =
                '\u{0}' as i32 as libc::c_char
        } else { flag = 0 as libc::c_int }
        code = atoi(cp);
        length =
            strlen((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr()) as
                libc::c_int;
        if length == code * 3 as libc::c_int ||
            flag != 0 && length > code * 3 as libc::c_int {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        0 as libc::c_int
    } else if strncmp(rule,
                      b"&\xe6\x95\xb0\xe5\xad\x97\xe9\x95\xb7:\x00" as
                          *const u8 as *const libc::c_char,
                      strlen(b"&\xe6\x95\xb0\xe5\xad\x97\xe9\x95\xb7:\x00" as
                          *const u8 as *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe6\x95\xb0\xe5\xad\x97\xe9\x95\xb7:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        code = atoi(cp) * 3 as libc::c_int;
        length =
            strlen((*(ptr2 as *mut MRPH_DATA)).Goi2.as_mut_ptr()) as
                libc::c_int;
        if length < code {
            /* &数字長 : 先頭の数字の長さ チェック (形態素レベル) */
            /* 形態素が指定長より短いとき */
            0 as libc::c_int
        } else {
            check_str_type((*(ptr2 as
                *mut MRPH_DATA)).Goi2.as_mut_ptr() as
                               *mut libc::c_uchar, 8 as libc::c_int,
                           code)
        }
    } else if strncmp(rule,
                      b"&\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe6\x9c\xab\xe5\xb0\xbe:\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe6\x9c\xab\xe5\xb0\xbe:\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe5\xbd\xa2\xe6\x85\x8b\xe7\xb4\xa0\xe6\x9c\xab\xe5\xb0\xbe:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        i =
            strlen((*(ptr2 as
                *mut MRPH_DATA)).Goi2.as_mut_ptr()).wrapping_sub(strlen(cp))
                as libc::c_int;
        if *cp as libc::c_int != 0 && i >= 0 as libc::c_int &&
            strcmp((*(ptr2 as
                *mut MRPH_DATA)).Goi2.as_mut_ptr().offset(i as
                isize),
                   cp) == 0 {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        0 as libc::c_int
    } else if strncmp(rule,
                      b"&\xe8\xa1\xa8\xe5\xb1\xa4:\x00" as *const u8 as
                          *const libc::c_char,
                      strlen(b"&\xe8\xa1\xa8\xe5\xb1\xa4:\x00" as *const u8 as
                          *const libc::c_char)) == 0 {
        if strcmp(rule.offset(strlen(b"&\xe8\xa1\xa8\xe5\xb1\xa4:\x00" as
            *const u8 as *const libc::c_char) as
            isize),
                  b"\xe7\x85\xa7\xe5\x90\x88\x00" as *const u8 as
                      *const libc::c_char) == 0 {
            cp =
                check_feature((*(ptr1 as *mut BNST_DATA)).f,
                              b"\xe4\xbf\x82\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char);
            if cp.is_null() { return 0 as libc::c_int; }
            if (*(ptr2 as
                *mut BNST_DATA)).SCASE_code[case2num(cp.offset(strlen(b"\xe4\xbf\x82:\x00"
                as
                *const u8
                as
                *const libc::c_char)
                as
                isize))
                as usize] != 0 {
                (0 as libc::c_int == 0) as libc::c_int
            } else { 0 as libc::c_int }
        } else if (*(ptr2 as
            *mut BNST_DATA)).SCASE_code[case2num(rule.offset(strlen(b"&\xe8\xa1\xa8\xe5\xb1\xa4:\x00"
            as
            *const u8
            as
            *const libc::c_char)
            as
            isize))
            as usize] != 0 {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else if strncmp(rule, b"&D:\x00" as *const u8 as *const libc::c_char,
                      strlen(b"&D:\x00" as *const u8 as *const libc::c_char))
        == 0 {
        if (ptr2 as
            *mut BNST_DATA).wrapping_offset_from(ptr1 as *mut BNST_DATA)
            as libc::c_long <=
            atoi(rule.offset(strlen(b"&D:\x00" as *const u8 as
                *const libc::c_char) as isize)) as
                libc::c_long {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else if strcmp(rule,
                     b"&\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab:\xe5\xbc\xb7\x00"
                         as *const u8 as *const libc::c_char) == 0 {
        subordinate_level_comp(ptr1 as *mut BNST_DATA,
                               ptr2 as *mut BNST_DATA)
    } else if strncmp(rule,
                      b"&\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab:\x00" as
                          *const u8 as *const libc::c_char,
                      strlen(b"&\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab:\x00" as
                          *const u8 as *const libc::c_char)) == 0 {
        subordinate_level_check(rule.offset(strlen(b"&\xe3\x83\xac\xe3\x83\x99\xe3\x83\xab:\x00"
            as *const u8 as
            *const libc::c_char)
            as isize), fd)
        /* &表層: 表層格チェック (文節レベル,係受レベル) */
        /* &D : 距離比較 (係受レベル) */
        /* &レベル:強 : 用言のレベル比較 (係受レベル) */
        /* &レベル:X : 用言がレベルX以上であるかどうか */
        /* (BNST_DATA *)ptr2); */
    } else if strncmp(rule,
                      b"&\xe4\xbf\x82\xe5\x81\xb4:\x00" as *const u8 as
                          *const libc::c_char,
                      strlen(b"&\xe4\xbf\x82\xe5\x81\xb4:\x00" as *const u8 as
                          *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe4\xbf\x82\xe5\x81\xb4:\x00" as *const u8
                as *const libc::c_char) as isize);
        if *cp as libc::c_int != '^' as i32 &&
            !check_feature((*(ptr1 as *mut BNST_DATA)).f, cp).is_null() ||
            *cp as libc::c_int == '^' as i32 &&
                check_feature((*(ptr1 as *mut BNST_DATA)).f,
                              cp.offset(1 as libc::c_int as
                                  isize)).is_null() {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else if strncmp(rule,
                      b"&\xe4\xbf\x82\xe5\x81\xb4\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf:\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe4\xbf\x82\xe5\x81\xb4\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf:\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe4\xbf\x82\xe5\x81\xb4\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        /* &係側 : 係側のFEATUREチェック (係受レベル) */
        /* &係側チェック : 係側のFEATUREチェック (文節ルール) */
        /* <PARA>ならばその子どもをみる */
        while !ptr2.is_null() &&
            (*(ptr2 as *mut BNST_DATA)).para_top_p as libc::c_int != 0 {
            ptr2 =
                (*(ptr2 as *mut BNST_DATA)).child[0 as libc::c_int as usize]
                    as *mut libc::c_void
        }
        i = 0 as libc::c_int;
        while !(*(ptr2 as *mut BNST_DATA)).child[i as usize].is_null() {
            if !check_feature((*(*(ptr2 as
                *mut BNST_DATA)).child[i as usize]).f,
                              cp).is_null() {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
            i += 1
        }
        0 as libc::c_int
    } else if strncmp(rule,
                      b"&\xe5\x8f\x97\xe5\x81\xb4\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf:\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe5\x8f\x97\xe5\x81\xb4\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf:\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe5\x8f\x97\xe5\x81\xb4\xe3\x83\x81\xe3\x82\xa7\xe3\x83\x83\xe3\x82\xaf:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        if !(*(ptr2 as *mut BNST_DATA)).parent.is_null() &&
            !check_feature((*(*(ptr2 as *mut BNST_DATA)).parent).f,
                           cp).is_null() {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else if strncmp(rule,
                      b"&\xe8\x87\xaa\xe7\xab\x8b\xe8\xaa\x9e\xe4\xb8\x80\xe8\x87\xb4\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe8\x87\xaa\xe7\xab\x8b\xe8\xaa\x9e\xe4\xb8\x80\xe8\x87\xb4\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        /* &受側チェック : 受側のFEATUREチェック (文節ルール) */
        /* &自立語一致 : 自立語が同じかどうか */
        /* if (!strcmp(((BNST_DATA *)ptr1)->head_ptr->Goi, 
	   ((BNST_DATA *)ptr2)->head_ptr->Goi)) { */
        if strcmp((*(ptr1 as *mut BNST_DATA)).Jiritu_Go.as_mut_ptr(),
                  (*(ptr2 as *mut BNST_DATA)).Jiritu_Go.as_mut_ptr()) == 0 {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else if strncmp(rule,
                      b"&\xe6\x96\x87\xe5\xad\x97\xe5\x88\x97\xe7\x85\xa7\xe5\x90\x88:\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe6\x96\x87\xe5\xad\x97\xe5\x88\x97\xe7\x85\xa7\xe5\x90\x88:\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe6\x96\x87\xe5\xad\x97\xe5\x88\x97\xe7\x85\xa7\xe5\x90\x88:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        if !strstr((*(ptr2 as *mut MRPH_DATA)).Goi.as_mut_ptr(), cp).is_null()
        {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else if strncmp(rule, b"&ST\x00" as *const u8 as *const libc::c_char,
                      strlen(b"&ST\x00" as *const u8 as *const libc::c_char))
        == 0 {
        (0 as libc::c_int == 0) as libc::c_int
    } else if strncmp(rule,
                      b"&OptCheck:\x00" as *const u8 as *const libc::c_char,
                      strlen(b"&OptCheck:\x00" as *const u8 as
                          *const libc::c_char)) == 0 {
        let mut opt: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        cp =
            rule.offset(strlen(b"&OptCheck:\x00" as *const u8 as
                *const libc::c_char) as isize);
        if *cp as libc::c_int == '-' as i32 {
            /* &文字列照合 : 原形との文字列部分マッチ by kuro 00/12/28 */
            /* &ST : 並列構造解析での類似度の閾値 (ここでは無視) */
            /* &OPTCHECK : オプションのチェック */
            /* '-'を含んでいたら飛ばす */
            cp = cp.offset(1)
        }
        opt = Options;
        while !(*opt).is_null() {
            if strcasecmp(cp, *opt) == 0 {
                return (0 as libc::c_int == 0) as libc::c_int;
            }
            opt = opt.offset(1)
        }
        0 as libc::c_int
    } else if strncmp(rule,
                      b"&\xe6\x85\x8b:\x00" as *const u8 as
                          *const libc::c_char,
                      strlen(b"&\xe6\x85\x8b:\x00" as *const u8 as
                          *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe6\x85\x8b:\x00" as *const u8 as
                *const libc::c_char) as isize);
        if strcmp(cp,
                  b"\xe8\x83\xbd\xe5\x8b\x95\x00" as *const u8 as
                      *const libc::c_char) == 0 &&
            (*(ptr2 as *mut BNST_DATA)).voice == 0 as libc::c_int ||
            strcmp(cp,
                   b"\xe5\x8f\x97\xe5\x8b\x95\x00" as *const u8 as
                       *const libc::c_char) == 0 &&
                ((*(ptr2 as *mut BNST_DATA)).voice & 2 as libc::c_int != 0
                    ||
                    (*(ptr2 as *mut BNST_DATA)).voice & 4 as libc::c_int
                        != 0) ||
            strcmp(cp,
                   b"\xe4\xbd\xbf\xe5\xbd\xb9\x00" as *const u8 as
                       *const libc::c_char) == 0 &&
                ((*(ptr2 as *mut BNST_DATA)).voice & 1 as libc::c_int != 0
                    ||
                    (*(ptr2 as *mut BNST_DATA)).voice & 4 as libc::c_int
                        != 0) {
            (0 as libc::c_int == 0) as libc::c_int
        } else { 0 as libc::c_int }
    } else if strcmp(rule,
                     b"&\xe8\xa8\x98\xe6\x86\xb6\x00" as *const u8 as
                         *const libc::c_char) == 0 {
        matched_ptr = ptr2;
        (0 as libc::c_int == 0) as libc::c_int
    } else if strncmp(rule,
                      b"&\xe5\x90\x8d\xe5\x8b\x95\xe7\x9b\xb8\xe4\xba\x92\xe6\x83\x85\xe5\xa0\xb1\xe9\x87\x8f:\x00"
                          as *const u8 as *const libc::c_char,
                      strlen(b"&\xe5\x90\x8d\xe5\x8b\x95\xe7\x9b\xb8\xe4\xba\x92\xe6\x83\x85\xe5\xa0\xb1\xe9\x87\x8f:\x00"
                          as *const u8 as *const libc::c_char)) == 0 {
        cp =
            rule.offset(strlen(b"&\xe5\x90\x8d\xe5\x8b\x95\xe7\x9b\xb8\xe4\xba\x92\xe6\x83\x85\xe5\xa0\xb1\xe9\x87\x8f:\x00"
                as *const u8 as *const libc::c_char) as
                isize);
        code = atoi(cp);
        check_nv_mi_parent_and_children(ptr2 as *mut TAG_DATA, code)
    } else { (0 as libc::c_int == 0) as libc::c_int };
}
/*
    else if (!strncmp(rule, "&時間", strlen("&時間"))) {
	if (sm_all_match(((BNST_DATA *)ptr2)->SM_code, "1128********")) {
	    return TRUE;
	}
	else {
	    return FALSE;
	}
    } */
/* &態 : 態をチェック */
/* &記憶 : 形態素または文節のポインタを記憶 */
/* &名動相互情報量 : 名詞動詞間の相互情報量のチェック (基本句ルール) */
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn feature_AND_match(mut fp: *mut FEATURE, mut fd: *mut FEATURE, mut p1: *mut libc::c_void, mut p2: *mut libc::c_void) -> libc::c_int {
    let mut value: libc::c_int = 0;
    while !fp.is_null() {
        if *(*fp).cp.offset(0 as libc::c_int as isize) as libc::c_int ==
            '^' as i32 &&
            *(*fp).cp.offset(1 as libc::c_int as isize) as libc::c_int ==
                '&' as i32 {
            value =
                check_function((*fp).cp.offset(1 as libc::c_int as isize), fd,
                               p1, p2);
            if value == (0 as libc::c_int == 0) as libc::c_int {
                return 0 as libc::c_int;
            }
        } else if *(*fp).cp.offset(0 as libc::c_int as isize) as libc::c_int
            == '&' as i32 {
            value = check_function((*fp).cp, fd, p1, p2);
            if value == 0 as libc::c_int { return 0 as libc::c_int; }
        } else if *(*fp).cp.offset(0 as libc::c_int as isize) as libc::c_int
            == '^' as i32 {
            if !check_feature(fd,
                              (*fp).cp.offset(1 as libc::c_int as
                                  isize)).is_null() {
                return 0 as libc::c_int;
            }
        } else if check_feature(fd, (*fp).cp).is_null() {
            return 0 as libc::c_int;
        }
        fp = (*fp).next
    }
    return (0 as libc::c_int == 0) as libc::c_int;
}
/*==================================================================*/
#[no_mangle]
pub unsafe extern "C" fn feature_pattern_match(mut fr: *mut FEATURE_PATTERN, mut fd: *mut FEATURE, mut p1: *mut libc::c_void, mut p2: *mut libc::c_void) -> libc::c_int {
    /* fr : ルール側のFEATURE_PATTERN,
       fd : データ側のFEATURE
       p1 : 係り受けの場合，係り側の構造体(MRPH_DATA,BNST_DATAなど)
       p2 : データ側の構造体(MRPH_DATA,BNST_DATAなど)
    */
    let mut i: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    /* PATTERNがなければマッチ */
    if (*fr).fp[0 as libc::c_int as usize].is_null() {
        return (0 as libc::c_int == 0) as libc::c_int;
    }
    /* ORの各条件を調べる */
    i = 0 as libc::c_int;
    while !(*fr).fp[i as usize].is_null() {
        value = feature_AND_match((*fr).fp[i as usize], fd, p1, p2);
        if value == (0 as libc::c_int == 0) as libc::c_int {
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*====================================================================*/
#[no_mangle]
pub unsafe extern "C" fn get_feature_for_chi(mut p_ptr: *mut BNST_DATA)
                                             -> *mut libc::c_char
/*====================================================================*/
{
    let mut feature: *mut libc::c_char = 0 as *mut libc::c_char;
    if !check_feature((*p_ptr).f,
                      b"AD\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char).is_null() {
        feature =
            b"AD\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"AS\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"AS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"BA\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"BA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"CC\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"CC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"CD\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"CD\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"CS\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"CS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"DEC\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"DEC\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"DEG\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"DEG\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"DER\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"DER\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"DEV\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"DEV\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"DT\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"DT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"ETC\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"ETC\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"FW\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"FW\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"IJ\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"IJ\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"JJ\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"JJ\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"LB\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"LB\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"LC\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"LC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"M\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"M\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"MSP\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"MSP\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"NN\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"NN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"NR\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"NR\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"NT\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"NT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"OD\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"OD\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"ON\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"ON\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"P\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"P\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"PN\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"PN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"PU\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"PU\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"SB\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"SB\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"SP\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"SP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"VV\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"VV\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"VA\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"VA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"VC\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"VC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if !check_feature((*p_ptr).f,
                             b"VE\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char).is_null() {
        feature =
            b"VE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        feature =
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    return feature;
}
/*====================================================================
                               END
====================================================================*/
