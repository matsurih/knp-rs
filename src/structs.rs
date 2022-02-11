use crate::ctools::{_IO_codecvt, _IO_marker, _IO_wide_data, cdb_rl, internal_state};
use crate::types::{__blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __jmp_buf, __mode_t, __nlink_t, __off64_t, __off_t, __syscall_slong_t, __time_t, __uid_t, alloc_func, BIN, BNST_DATA, Bytef, CASE_COMPONENT, CASE_FRAME, CF_PRED_MGR, CF_ptr, CHECK_DATA, CKYptr, CPM_ptr, DBM_FILE, E_FEATURES, ELLIPSIS_CMM, ELLIPSIS_COMPONENT, ENTITY, FEATURE, FEATUREptr, free_func, in_addr_t, in_port_t, MENTION, MENTION_MGR, PARA_DATA, Para_M_ptr, PARA_MANAGER, sa_family_t, SENTENCE_DATA, size_t, TAG_DATA, Treeptr_B, uInt, uLong, voidpf};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CLASS {
    pub id: *mut libc::c_uchar,
    pub cost: libc::c_int,
    pub kt: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdb_make {
    pub cdb_fd: libc::c_int,
    pub cdb_dpos: libc::c_uint,
    pub cdb_rcnt: libc::c_uint,
    pub cdb_buf: [libc::c_uchar; 4096],
    pub cdb_bpos: *mut libc::c_uchar,
    pub cdb_rec: [*mut cdb_rl; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FEATURE {
    pub cp: *mut libc::c_char,
    pub next: FEATUREptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FORM {
    pub name: *mut libc::c_uchar,
    pub gobi: *mut libc::c_uchar,
    pub gobi_yomi: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FEATURE_PATTERN {
    pub fp: [*mut FEATURE; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REGEXPMRPH {
    pub type_flag: libc::c_char,
    pub ast_flag: libc::c_char,
    pub Hinshi_not: libc::c_char,
    pub Hinshi: [libc::c_int; 64],
    pub Bunrui_not: libc::c_char,
    pub Bunrui: [libc::c_int; 64],
    pub Kata_not: libc::c_char,
    pub Katuyou_Kata: [libc::c_int; 64],
    pub Kei_not: libc::c_char,
    pub Katuyou_Kei: [*mut libc::c_char; 64],
    pub Goi_not: libc::c_char,
    pub Goi: [*mut libc::c_char; 64],
    pub f_pattern: FEATURE_PATTERN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REGEXPMRPHS {
    pub mrph: *mut REGEXPMRPH,
    pub mrphsize: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REGEXPBNST {
    pub type_flag: libc::c_char,
    pub ast_flag: libc::c_char,
    pub mrphs: *mut REGEXPMRPHS,
    pub f_pattern: FEATURE_PATTERN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REGEXPBNSTS {
    pub bnst: *mut REGEXPBNST,
    pub bnstsize: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MrphRule {
    pub pre_pattern: *mut REGEXPMRPHS,
    pub self_pattern: *mut REGEXPMRPHS,
    pub post_pattern: *mut REGEXPMRPHS,
    pub f: *mut FEATURE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DpndRule {
    pub dependant: FEATURE_PATTERN,
    pub governor: [FEATURE_PATTERN; 35],
    pub dpnd_type: [libc::c_char; 35],
    pub barrier: FEATURE_PATTERN,
    pub preference: libc::c_int,
    pub decide: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuleVector {
    pub file: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub mode: libc::c_int,
    pub breakmode: libc::c_int,
    pub direction: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHI_DPND {
    pub direction: [libc::c_char; 10],
    pub prob_LtoR: [libc::c_double; 10],
    pub prob_RtoL: [libc::c_double; 10],
    pub prob_pos_LtoR: libc::c_double,
    pub prob_pos_RtoL: libc::c_double,
    pub type_0: [[libc::c_char; 10]; 10],
    pub occur_pos: libc::c_double,
    pub prob_dis_comma_LtoR: [libc::c_double; 10],
    pub prob_dis_comma_RtoL: [libc::c_double; 10],
    pub lex_prob_LtoR: [libc::c_double; 10],
    pub lex_prob_RtoL: [libc::c_double; 10],
    pub lex_prob_dis_comma_LtoR: [libc::c_double; 10],
    pub lex_prob_dis_comma_RtoL: [libc::c_double; 10],
    pub left_pos_index: [libc::c_int; 10],
    pub right_pos_index: [libc::c_int; 10],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHI_POS {
    pub pos: [*mut libc::c_char; 33],
    pub prob: [libc::c_double; 33],
    pub prob_pos_index: [libc::c_double; 33],
    pub pos_index: [libc::c_int; 33],
    pub pos_max: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHI_ROOT {
    pub prob: [libc::c_double; 33],
    pub pos_index: [libc::c_int; 33],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_manager {
    pub num: libc::c_int,
    pub entity: [ENTITY; 4096],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TYPE {
    pub name: *mut libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CDB_FILE {
    pub fd: libc::c_int,
    pub mode: libc::c_int,
    pub cdbm: cdb_make,
    pub cdb: cdb,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct QUOTE_DATA {
    pub in_num: [libc::c_int; 40],
    pub out_num: [libc::c_int; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tnode_b {
    pub type_0: libc::c_int,
    pub num: libc::c_int,
    pub parent: Treeptr_B,
    pub child: [Treeptr_B; 32],
    pub length: libc::c_int,
    pub space: libc::c_int,
    pub dpnd_head: libc::c_int,
    pub dpnd_type: libc::c_char,
    pub dpnd_dflt: libc::c_int,
    pub para_num: libc::c_int,
    pub para_key_type: libc::c_char,
    pub para_top_p: libc::c_char,
    pub para_type: libc::c_char,
    pub to_para_p: libc::c_char,
    pub sp_level: libc::c_int,
    pub mrph_num: libc::c_int,
    pub preserve_mrph_num: libc::c_int,
    pub mrph_ptr: *mut MRPH_DATA,
    pub head_ptr: *mut MRPH_DATA,
    pub BGH_code: [libc::c_char; 2817],
    pub BGH_num: libc::c_int,
    pub SM_code: [libc::c_char; 3073],
    pub SM_num: libc::c_int,
    pub voice: libc::c_int,
    pub cf_num: libc::c_int,
    pub cf_ptr: CF_ptr,
    pub cpm_ptr: CPM_ptr,
    pub pred_num: libc::c_int,
    pub f: FEATUREptr,
    pub pred_b_ptr: *mut tnode_b,
    pub is_para: libc::c_int,
    pub SCASE_code: [libc::c_char; 11],
    pub Jiritu_Go: [libc::c_char; 256],
    pub dpnd_rule: *mut DpndRule,
    pub tag_ptr: *mut tnode_t,
    pub tag_num: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tnode_t {
    pub type_0: libc::c_int,
    pub num: libc::c_int,
    pub parent: *mut tnode_t,
    pub child: [*mut tnode_t; 32],
    pub length: libc::c_int,
    pub space: libc::c_int,
    pub dpnd_head: libc::c_int,
    pub dpnd_type: libc::c_char,
    pub dpnd_dflt: libc::c_int,
    pub para_num: libc::c_int,
    pub para_key_type: libc::c_char,
    pub para_top_p: libc::c_char,
    pub para_type: libc::c_char,
    pub to_para_p: libc::c_char,
    pub sp_level: libc::c_int,
    pub mrph_num: libc::c_int,
    pub preserve_mrph_num: libc::c_int,
    pub mrph_ptr: *mut MRPH_DATA,
    pub head_ptr: *mut MRPH_DATA,
    pub BGH_code: [libc::c_char; 2817],
    pub BGH_num: libc::c_int,
    pub SM_code: [libc::c_char; 3073],
    pub SM_num: libc::c_int,
    pub voice: libc::c_int,
    pub cf_num: libc::c_int,
    pub cf_ptr: CF_ptr,
    pub cpm_ptr: CPM_ptr,
    pub pred_num: libc::c_int,
    pub f: FEATUREptr,
    pub pred_b_ptr: *mut tnode_t,
    pub is_para: libc::c_int,
    pub SCASE_code: [libc::c_char; 11],
    pub bnum: libc::c_int,
    pub inum: libc::c_int,
    pub b_ptr: *mut BNST_DATA,
    pub settou_num: libc::c_int,
    pub jiritu_num: libc::c_int,
    pub fuzoku_num: libc::c_int,
    pub settou_ptr: *mut MRPH_DATA,
    pub jiritu_ptr: *mut MRPH_DATA,
    pub fuzoku_ptr: *mut MRPH_DATA,
    pub e_cf_num: libc::c_int,
    pub c_cpm_ptr: CPM_ptr,
    pub next: *mut tnode_t,
    pub mention_mgr: MENTION_MGR,
    pub tcf_ptr: *mut tcf_def,
    pub ctm_ptr: *mut ctm_def,
    pub score_diff: libc::c_double,
    pub ga_score_diff: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctm_def {
    pub score: libc::c_double,
    pub score_def: libc::c_double,
    pub case_analysis_score: libc::c_double,
    pub cf_ptr: *mut CASE_FRAME,
    pub filled_element: [libc::c_int; 24],
    pub filled_entity: [libc::c_int; 4096],
    pub non_match_element: [libc::c_int; 24],
    pub result_num: libc::c_int,
    pub case_result_num: libc::c_int,
    pub annotated_result_num: libc::c_int,
    pub cf_element_num: [libc::c_int; 24],
    pub tcf_element_num: [libc::c_int; 24],
    pub tcf_element_num_functional: [libc::c_int; 24],
    pub elem_b_ptr: [*mut TAG_DATA; 24],
    pub entity_num: [libc::c_int; 24],
    pub type_0: [libc::c_char; 24],
    pub ga_entity: libc::c_int,
    pub case_analysis_ga_entity: libc::c_int,
    pub overt_arguments_score: libc::c_double,
    pub all_arguments_score: libc::c_double,
    pub omit_feature: [[libc::c_double; 9152]; 4],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_def {
    pub type_0: libc::c_int,
    pub type_flag: libc::c_int,
    pub element_num: libc::c_int,
    pub oblig: [libc::c_int; 24],
    pub adjacent: [libc::c_int; 24],
    pub pp: [[libc::c_int; 10]; 24],
    pub sp: [libc::c_int; 24],
    pub pp_str: [*mut libc::c_char; 24],
    pub sm: [*mut libc::c_char; 24],
    pub sm_delete: [*mut libc::c_char; 24],
    pub sm_delete_size: [libc::c_int; 24],
    pub sm_delete_num: [libc::c_int; 24],
    pub sm_specify: [*mut libc::c_char; 24],
    pub sm_specify_size: [libc::c_int; 24],
    pub sm_specify_num: [libc::c_int; 24],
    pub ex: [*mut libc::c_char; 24],
    pub ex_list: [*mut *mut libc::c_char; 24],
    pub ex_freq: [*mut libc::c_int; 24],
    pub ex_size: [libc::c_int; 24],
    pub ex_num: [libc::c_int; 24],
    pub freq: [libc::c_int; 24],
    pub semantics: [*mut libc::c_char; 24],
    pub gex_list: [*mut *mut libc::c_char; 24],
    pub gex_freq: [*mut libc::c_double; 24],
    pub gex_size: [libc::c_int; 24],
    pub gex_num: [libc::c_int; 24],
    pub voice: libc::c_int,
    pub cf_address: libc::c_ulonglong,
    pub cf_size: libc::c_int,
    pub cf_id: [libc::c_char; 280],
    pub pred_type: [libc::c_char; 4],
    pub entry: *mut libc::c_char,
    pub imi: [libc::c_char; 128],
    pub etcflag: libc::c_int,
    pub feature: *mut libc::c_char,
    pub weight: [libc::c_int; 24],
    pub samecase: [[libc::c_int; 2]; 24],
    pub cf_align: [CF_ALIGNMENT; 5],
    pub pred_b_ptr: *mut TAG_DATA,
    pub cf_similarity: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CF_ALIGNMENT {
    pub cf_id: *mut libc::c_char,
    pub aligned_case: [[libc::c_int; 2]; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tcf_def {
    pub cf: CASE_FRAME,
    pub cf_with_functional_tag: [CASE_FRAME; 5],
    pub cf_num: libc::c_int,
    pub pred_b_ptr: *mut TAG_DATA,
    pub elem_b_ptr: [*mut TAG_DATA; 24],
    pub map_tcf_elem_to_cf: [libc::c_int; 24],
    pub map_tcf_elem_to_cf_elem: [libc::c_int; 24],
    pub elem_b_num: [libc::c_int; 24],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mention_manager {
    pub num: libc::c_int,
    pub cf_id: [libc::c_char; 280],
    pub cf_ptr: CF_ptr,
    pub mention: [MENTION; 8],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mention {
    pub sent_num: libc::c_int,
    pub tag_num: libc::c_int,
    pub cpp_string: [libc::c_char; 16],
    pub spp_string: [libc::c_char; 16],
    pub type_0: libc::c_char,
    pub salience_score: libc::c_double,
    pub static_salience_score: libc::c_double,
    pub tag_ptr: *mut tnode_t,
    pub entity: *mut entity,
    pub explicit_mention: *mut mention,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity {
    pub num: libc::c_int,
    pub output_num: libc::c_int,
    pub mentioned_num: libc::c_int,
    pub link_entity: libc::c_int,
    pub first_appearance: libc::c_int,
    pub salience_score: libc::c_double,
    pub salience_mem: libc::c_double,
    pub tmp_salience_flag: libc::c_int,
    pub hypothetical_flag: libc::c_int,
    pub real_entity: libc::c_int,
    pub hypothetical_entity: libc::c_int,
    pub skip_flag: libc::c_int,
    pub hypothetical_name: [libc::c_char; 129],
    pub mention: [*mut MENTION; 256],
    pub named_entity: [libc::c_char; 128],
    pub name: [libc::c_char; 129],
    pub corefer_id: libc::c_int,
    pub rep_sen_num: libc::c_int,
    pub rep_tag_num: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpm_def {
    pub cf: CASE_FRAME,
    pub pred_b_ptr: *mut TAG_DATA,
    pub elem_b_ptr: [*mut TAG_DATA; 24],
    pub para_b_ptr: [*mut TAG_DATA; 24],
    pub elem_s_ptr: [*mut sentence; 24],
    pub elem_b_num: [libc::c_int; 24],
    pub score: libc::c_double,
    pub result_num: libc::c_int,
    pub tie_num: libc::c_int,
    pub cmm: [CF_MATCH_MGR; 5],
    pub decided: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CF_MATCH_MGR {
    pub cf_ptr: *mut CASE_FRAME,
    pub score: libc::c_double,
    pub pure_score: [libc::c_double; 10],
    pub sufficiency: libc::c_double,
    pub result_num: libc::c_int,
    pub result_lists_p: [LIST; 10],
    pub result_lists_d: [LIST; 10],
    pub cpm: *mut cpm_def,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LIST {
    pub flag: [libc::c_int; 24],
    pub score: [libc::c_double; 24],
    pub pos: [libc::c_int; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentence {
    pub Sen_num: libc::c_int,
    pub available: libc::c_int,
    pub Mrph_num: libc::c_int,
    pub New_Mrph_num: libc::c_int,
    pub Bnst_num: libc::c_int,
    pub New_Bnst_num: libc::c_int,
    pub Max_New_Bnst_num: libc::c_int,
    pub Tag_num: libc::c_int,
    pub New_Tag_num: libc::c_int,
    pub Para_M_num: libc::c_int,
    pub Para_num: libc::c_int,
    pub frame_num_max: libc::c_int,
    pub mrph_data: *mut MRPH_DATA,
    pub bnst_data: *mut BNST_DATA,
    pub tag_data: *mut TAG_DATA,
    pub para_data: *mut PARA_DATA,
    pub para_manager: *mut PARA_MANAGER,
    pub cpm: *mut CF_PRED_MGR,
    pub cf: *mut CASE_FRAME,
    pub Best_mgr: *mut TOTAL_MGR,
    pub KNPSID: *mut libc::c_char,
    pub Comment: *mut libc::c_char,
    pub score: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TOTAL_MGR {
    pub dpnd: DPND,
    pub pssb: libc::c_int,
    pub dflt: libc::c_int,
    pub score: libc::c_double,
    pub pred_num: libc::c_int,
    pub cpm: [CF_PRED_MGR; 64],
    pub ID: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DPND {
    pub head: [libc::c_int; 200],
    pub type_0: [libc::c_char; 200],
    pub dflt: [libc::c_int; 200],
    pub mask: [libc::c_int; 200],
    pub pos: libc::c_int,
    pub check: [CHECK_DATA; 200],
    pub f: [*mut FEATURE; 200],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _check {
    pub num: libc::c_int,
    pub def: libc::c_int,
    pub pos: [libc::c_int; 200],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_para_manager {
    pub para_num: libc::c_int,
    pub para_data_num: [libc::c_int; 32],
    pub part_num: libc::c_int,
    pub start: [libc::c_int; 32],
    pub end: [libc::c_int; 32],
    pub parent: Para_M_ptr,
    pub child: [Para_M_ptr; 32],
    pub child_num: libc::c_int,
    pub bnst_ptr: *mut BNST_DATA,
    pub status: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BnstRule {
    pub pre_pattern: *mut REGEXPBNSTS,
    pub self_pattern: *mut REGEXPBNSTS,
    pub post_pattern: *mut REGEXPBNSTS,
    pub f: *mut FEATURE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tnode_p {
    pub para_char: libc::c_char,
    pub type_0: libc::c_int,
    pub max_num: libc::c_int,
    pub key_pos: libc::c_int,
    pub iend_pos: libc::c_int,
    pub jend_pos: libc::c_int,
    pub max_path: [libc::c_int; 200],
    pub f_pattern: FEATURE_PATTERN,
    pub max_score: libc::c_float,
    pub pure_score: libc::c_float,
    pub status: libc::c_char,
    pub manager_ptr: Para_M_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MRPH_DATA {
    pub type_0: libc::c_int,
    pub num: libc::c_int,
    pub parent: Treeptr_B,
    pub child: [Treeptr_B; 32],
    pub length: libc::c_int,
    pub space: libc::c_int,
    pub dpnd_head: libc::c_int,
    pub dpnd_type: libc::c_char,
    pub dpnd_dflt: libc::c_int,
    pub para_num: libc::c_int,
    pub para_key_type: libc::c_char,
    pub para_top_p: libc::c_char,
    pub para_type: libc::c_char,
    pub to_para_p: libc::c_char,
    pub tnum: libc::c_int,
    pub inum: libc::c_int,
    pub out_head_flag: libc::c_int,
    pub Goi: [libc::c_char; 129],
    pub Yomi: [libc::c_char; 129],
    pub Goi2: [libc::c_char; 129],
    pub Hinshi: libc::c_int,
    pub Bunrui: libc::c_int,
    pub Katuyou_Kata: libc::c_int,
    pub Katuyou_Kei: libc::c_int,
    pub Imi: [libc::c_char; 1024],
    pub f: FEATUREptr,
    pub Num: libc::c_int,
    pub SM: *mut libc::c_char,
    pub Pos: [libc::c_char; 4],
    pub Type: [libc::c_char; 9],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct thesaurus {
    pub path: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub format: *mut libc::c_int,
    pub code_size: libc::c_int,
    pub exist: libc::c_int,
    pub db: DBM_FILE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BIN {
    pub car: *mut libc::c_void,
    pub cdr: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CELL {
    pub tag: libc::c_int,
    pub value: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub cons: BIN,
    pub atom: *mut libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_list {
    pub key: *mut libc::c_char,
    pub cfid: *mut *mut libc::c_char,
    pub cfid_num: libc::c_int,
    pub cfid_max: libc::c_int,
    pub next: *mut cf_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ellipsis_list {
    pub score: libc::c_float,
    pub pure_score: libc::c_float,
    pub cc: [ELLIPSIS_COMPONENT; 50],
    pub f: FEATUREptr,
    pub result_num: libc::c_int,
    pub ecmm: [ELLIPSIS_CMM; 5],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ellipsis_cmm_list {
    pub cmm: CF_MATCH_MGR,
    pub cpm: CF_PRED_MGR,
    pub element_num: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ellipsis_component {
    pub s: *mut SENTENCE_DATA,
    pub pp_str: *mut libc::c_char,
    pub bnst: libc::c_int,
    pub score: libc::c_float,
    pub dist: libc::c_int,
    pub next: *mut ellipsis_component,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CKY {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub cp: libc::c_char,
    pub score: libc::c_double,
    pub para_score: libc::c_double,
    pub chicase_score: libc::c_double,
    pub chicase_lex_score: libc::c_double,
    pub para_flag: libc::c_int,
    pub dpnd_type: libc::c_char,
    pub direction: libc::c_int,
    pub index: libc::c_int,
    pub b_ptr: *mut BNST_DATA,
    pub scase_check: [libc::c_int; 11],
    pub un_count: libc::c_int,
    pub cpm_ptr: *mut CF_PRED_MGR,
    pub left: CKYptr,
    pub right: CKYptr,
    pub next: CKYptr,
    pub left_pos_index: libc::c_int,
    pub right_pos_index: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sm_list {
    pub key: *mut libc::c_char,
    pub sm: *mut libc::c_char,
    pub next: *mut sm_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dtcond {
    pub num: libc::c_int,
    pub eq: libc::c_int,
    pub value: libc::c_float,
    pub next: *mut _dtcond,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DT {
    pub ContextRules: [DTRULE; 1000],
    pub ContextRuleNum: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct KoouRule {
    pub start_pattern: *mut REGEXPMRPHS,
    pub end_pattern: *mut REGEXPMRPHS,
    pub uke_pattern: *mut REGEXPMRPHS,
    pub dpnd_type: libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HomoRule {
    pub pre_pattern: *mut REGEXPMRPHS,
    pub pattern: *mut REGEXPMRPHS,
    pub f: *mut FEATURE,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeneralRuleType {
    pub RuleArray: *mut libc::c_void,
    pub CurRuleSize: libc::c_int,
    pub type_0: libc::c_int,
    pub mode: libc::c_int,
    pub breakmode: libc::c_int,
    pub direction: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPAL_TRANS_FRAME {
    pub DATA: [libc::c_uchar; 8192000],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct case_component {
    pub word: *mut libc::c_char,
    pub pp_str: *mut libc::c_char,
    pub sent_num: libc::c_int,
    pub tag_num: libc::c_int,
    pub count: libc::c_int,
    pub flag: libc::c_int,
    pub next: *mut case_component,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct predicate_anaphora_list {
    pub key: *mut libc::c_char,
    pub voice: libc::c_int,
    pub cf_addr: libc::c_int,
    pub cc: [*mut CASE_COMPONENT; 20],
    pub next: *mut predicate_anaphora_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ellipsis_features {
    pub ellipsis_class: libc::c_int,
    pub similarity: libc::c_float,
    pub event1: libc::c_float,
    pub event2: libc::c_float,
    pub pos: libc::c_int,
    pub frequency: libc::c_int,
    pub discourse_depth: libc::c_int,
    pub refered_num_surface: libc::c_float,
    pub refered_num_ellipsis: libc::c_float,
    pub c_pp: libc::c_int,
    pub c_distance: libc::c_int,
    pub c_dist_bnst: libc::c_int,
    pub c_fs_flag: libc::c_int,
    pub c_location: libc::c_int,
    pub c_topic_flag: libc::c_int,
    pub c_no_topic_flag: libc::c_int,
    pub c_in_cnoun_flag: libc::c_int,
    pub c_subject_flag: libc::c_int,
    pub c_dep_mc_flag: libc::c_int,
    pub c_n_modify_flag: libc::c_int,
    pub c_dep_p_level: [libc::c_char; 3],
    pub c_prev_p_flag: libc::c_int,
    pub c_get_over_p_flag: libc::c_int,
    pub c_sm_none_flag: libc::c_int,
    pub c_extra_tag: libc::c_int,
    pub p_pp: libc::c_int,
    pub p_voice: libc::c_int,
    pub p_type: libc::c_int,
    pub p_sahen_flag: libc::c_int,
    pub p_cf_subject_flag: libc::c_int,
    pub p_cf_sentence_flag: libc::c_int,
    pub p_n_modify_flag: libc::c_int,
    pub p_dep_p_level: [libc::c_char; 3],
    pub c_ac: libc::c_int,
    pub match_sm_flag: libc::c_int,
    pub match_case: libc::c_int,
    pub match_verb: libc::c_int,
    pub utype: libc::c_int,
    pub objectrecognition: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ellipsis_svm_features {
    pub similarity: libc::c_float,
    pub frequency: libc::c_float,
    pub discourse_depth_inverse: libc::c_float,
    pub refered_num_surface: libc::c_float,
    pub refered_num_ellipsis: libc::c_float,
    pub c_pp: [libc::c_int; 44],
    pub c_location: [libc::c_int; 21],
    pub c_distance: libc::c_int,
    pub c_fs_flag: libc::c_int,
    pub c_topic_flag: libc::c_int,
    pub c_no_topic_flag: libc::c_int,
    pub c_in_cnoun_flag: libc::c_int,
    pub c_subject_flag: libc::c_int,
    pub c_n_modify_flag: libc::c_int,
    pub c_dep_mc_flag: libc::c_int,
    pub c_dep_p_level: [libc::c_int; 6],
    pub c_prev_p_flag: libc::c_int,
    pub c_get_over_p_flag: libc::c_int,
    pub c_sm_none_flag: libc::c_int,
    pub c_extra_tag: [libc::c_int; 3],
    pub p_pp: [libc::c_int; 3],
    pub p_voice: [libc::c_int; 3],
    pub p_type: [libc::c_int; 3],
    pub p_sahen_flag: libc::c_int,
    pub p_cf_subject_flag: libc::c_int,
    pub p_cf_sentence_flag: libc::c_int,
    pub p_n_modify_flag: libc::c_int,
    pub match_case: libc::c_int,
    pub match_verb: libc::c_int,
    pub utype: [libc::c_int; 12],
    pub objectrecognition: libc::c_int,
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct ellipsis_twin_cand_svm_features {
    pub c1_similarity: libc::c_float,
    pub c2_similarity: libc::c_float,
    pub c1_pp: [libc::c_int; 44],
    pub c1_location: [libc::c_int; 21],
    pub c1_fs_flag: libc::c_int,
    pub c1_topic_flag: libc::c_int,
    pub c1_no_topic_flag: libc::c_int,
    pub c1_in_cnoun_flag: libc::c_int,
    pub c1_subject_flag: libc::c_int,
    pub c1_n_modify_flag: libc::c_int,
    pub c1_dep_mc_flag: libc::c_int,
    pub c1_dep_p_level: [libc::c_int; 6],
    pub c1_prev_p_flag: libc::c_int,
    pub c1_get_over_p_flag: libc::c_int,
    pub c1_sm_none_flag: libc::c_int,
    pub c1_extra_tag: [libc::c_int; 3],
    pub c2_pp: [libc::c_int; 44],
    pub c2_location: [libc::c_int; 21],
    pub c2_fs_flag: libc::c_int,
    pub c2_topic_flag: libc::c_int,
    pub c2_no_topic_flag: libc::c_int,
    pub c2_in_cnoun_flag: libc::c_int,
    pub c2_subject_flag: libc::c_int,
    pub c2_n_modify_flag: libc::c_int,
    pub c2_dep_mc_flag: libc::c_int,
    pub c2_dep_p_level: [libc::c_int; 6],
    pub c2_prev_p_flag: libc::c_int,
    pub c2_get_over_p_flag: libc::c_int,
    pub c2_sm_none_flag: libc::c_int,
    pub c2_extra_tag: [libc::c_int; 3],
    pub p_pp: [libc::c_int; 3],
    pub p_voice: [libc::c_int; 3],
    pub p_type: [libc::c_int; 3],
    pub p_sahen_flag: libc::c_int,
    pub p_cf_subject_flag: libc::c_int,
    pub p_cf_sentence_flag: libc::c_int,
    pub p_n_modify_flag: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ellipsis_candidate {
    pub ef: *mut E_FEATURES,
    pub s: *mut SENTENCE_DATA,
    pub tp: *mut TAG_DATA,
    pub tag: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_list {
    pub key: *mut libc::c_char,
    pub surface_num: libc::c_double,
    pub ellipsis_num: libc::c_double,
    pub next: *mut entity_list,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PP_STR_TO_CODE {
    pub hstr: *mut libc::c_char,
    pub kstr: *mut libc::c_char,
    pub code: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpm_cache {
    pub key: *mut libc::c_char,
    pub cpm: *mut CF_PRED_MGR,
    pub next: *mut cpm_cache,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sort_kv {
    pub key: libc::c_int,
    pub value: libc::c_int,
}