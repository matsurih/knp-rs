/*====================================================================

				EXTERN

                                               S.Kurohashi 93. 5.31

    $Id$
====================================================================*/

extern int		Thesaurus;
extern int		ParaThesaurus;
extern THESAURUS_FILE	THESAURUS[];

extern int		Revised_para_num;

extern char		*ErrorComment;
extern char 		PM_Memo[];

extern int		BGHExist;
extern int		SMExist;
extern int		SM2CODEExist;
extern int		SMP2SMGExist;
DBM_FILE		sm_db;
DBM_FILE		sm2code_db;
DBM_FILE		smp2smg_db;

extern SENTENCE_DATA	sentence_data[];

extern int 		match_matrix[][BNST_MAX];
extern int 		path_matrix[][BNST_MAX];
extern int		restrict_matrix[][BNST_MAX];
extern int 		Dpnd_matrix[][BNST_MAX];
extern int 		Quote_matrix[][BNST_MAX];
extern int 		Mask_matrix[][BNST_MAX];

extern char		**Options;
extern int 		OptAnalysis;
extern int 		OptInput;
extern int 		OptExpress;
extern int 		OptDisplay;
extern int 		OptExpandP;
extern int		OptCheck;
extern int		OptUseScase;
extern char		OptIgnoreChar;
extern int		OptReadFeature;
extern int		OptCopula;
extern VerboseType	VerboseLevel;

extern CLASS    	Class[CLASSIFY_NO + 1][CLASSIFY_NO + 1];
extern TYPE     	Type[TYPE_NO];
extern FORM     	Form[TYPE_NO][FORM_NO];
extern int 		CLASS_num;

extern HomoRule 	HomoRuleArray[];
extern int 		CurHomoRuleSize;

extern KoouRule		KoouRuleArray[];
extern int		CurKoouRuleSize;
extern DpndRule		DpndRuleArray[];
extern int		CurDpndRuleSize;

extern BnstRule		ContRuleArray[];
extern int 		ContRuleSize;

void			*EtcRuleArray;
int			CurEtcRuleSize;

extern char 		*Case_name[];

extern SMLIST	smlist[];


/* 関数プロトタイプ */

/* bnst_compare.c */
extern int subordinate_level_check(char *cp, BNST_DATA *ptr2);
extern int levelcmp(char *cp1, char *cp2);
extern int subordinate_level_comp(BNST_DATA *ptr1, BNST_DATA *ptr2);
extern int subordinate_level_forbid(char *cp, BNST_DATA *ptr2);
extern void calc_match_matrix(SENTENCE_DATA *sp);

/* configfile.c */
extern char *check_dict_filename(char *file, int flag);
extern char *check_rule_filename(char *file);
extern void init_configfile(char *opfile);
extern void server_read_rc(FILE *fp);

/* db.c */
extern char *db_get(DBM_FILE db, char *buf);
extern DBM_FILE db_read_open(char *filename);

/* dpnd_analysis.c */
extern void dpnd_info_to_bnst(SENTENCE_DATA *sp, DPND *dp);
extern void dpnd_info_to_tag(SENTENCE_DATA *sp, DPND *dp);
extern int compare_dpnd(SENTENCE_DATA *sp, TOTAL_MGR *new_mgr, TOTAL_MGR *best_mgr);
extern int after_decide_dpnd(SENTENCE_DATA *sp);
extern void calc_dpnd_matrix(SENTENCE_DATA *sp);
extern int relax_dpnd_matrix(SENTENCE_DATA *sp);
extern void para_postprocess(SENTENCE_DATA *sp);
extern int detect_dpnd_case_struct(SENTENCE_DATA *sp);
extern void when_no_dpnd_struct(SENTENCE_DATA *sp);
extern void check_candidates(SENTENCE_DATA *sp);
extern void memo_by_program(SENTENCE_DATA *sp);

/* feature.c */
extern char *check_feature(FEATURE *fp, char *fname);
extern void assign_cfeature(FEATURE **fpp, char *fname);
extern int feature_pattern_match(FEATURE_PATTERN *fr, FEATURE *fd, void *p1, void *p2);
extern void print_one_feature(char *cp, FILE *filep);
extern void print_feature(FEATURE *fp, FILE *filep);
extern int comp_feature(char *data, char *pattern);
extern void print_feature2(FEATURE *fp, FILE *filep);
extern void print_some_feature(FEATURE *fp, FILE *filep);
extern int feature_AND_match(FEATURE *fp, FEATURE *fd, void *p1, void *p2);
extern void string2feature_pattern(FEATURE_PATTERN *f, char *cp);
extern void assign_feature(FEATURE **fpp1, FEATURE **fpp2, void *ptr);
extern void list2feature_pattern(FEATURE_PATTERN *f, CELL *cell);
extern void list2feature(CELL *cp, FEATURE **fpp);
extern void clear_feature(FEATURE **fpp);
extern void append_feature(FEATURE **fpp, FEATURE *afp);
extern void delete_cfeature(FEATURE **fpp, char *type);
extern void copy_feature(FEATURE **dst_fpp, FEATURE *src_fp);

/* koou.c */
int koou(SENTENCE_DATA *sp);

/* lib_bgh.c */
extern char *_get_bgh(char *cp, char *arg);
extern int bgh_code_match(char *c1, char *c2);
extern int bgh_code_match_for_case(char *cp1, char *cp2);
extern void init_bgh();
extern void close_bgh();

/* lib_dt.c */
extern float dt_classify(char *data, int pp);

/* lib_event.c */
extern float get_event_value(SENTENCE_DATA *sp1, TAG_DATA *p1, 
			     SENTENCE_DATA *sp2, TAG_DATA *p2);
extern float get_cf_event_value(CASE_FRAME *cf1, CASE_FRAME *cf2);

/* lib_lib.c */
extern int str_part_eq(char *dat, char *pat);

/* lib_print.c */
extern void print_kakari(SENTENCE_DATA *sp, int type);
extern void _print_bnst(TAG_DATA *ptr);
extern void print_matrix(SENTENCE_DATA *sp, int type, int L_B);
extern void print_result(SENTENCE_DATA *sp);
extern void print_bnst(BNST_DATA *ptr, char *cp);
extern void check_bnst(SENTENCE_DATA *sp);
extern void print_para_relation(SENTENCE_DATA *sp);
extern void assign_para_similarity_feature(SENTENCE_DATA *sp);
extern void prepare_all_entity(SENTENCE_DATA *sp);

/* lib_scase.c */
extern void set_pred_voice(BNST_DATA *b_ptr);
extern void get_scase_code(BNST_DATA *ptr);
extern char *make_pred_string(TAG_DATA *t_ptr);
extern void init_scase();
extern void close_scase();

/* lib_sm.c */
extern char *_get_ntt(char *cp, char *arg);
extern char *_process_code(char *code, char *arg);
extern int _code_match(char *code1,char *code2);
extern char *_code_flow(char *code);
extern char *sm2code(char *cp);
extern char *code2sm(char *cp);
extern float ntt_code_match(char *c1, char *c2, int flag);
extern int comp_sm(char *cpp, char *cpd, int start);
extern int _sm_match_score(char *cpp, char *cpd, int flag);
extern int sm_time_match(char *c);
extern void init_ntt();
extern void close_ntt();
extern char *_smp2smg(char *cp);
extern char *smp2smg(char *cpd, int flag);
extern void merge_smp2smg(BNST_DATA *bp);
extern int sm_match_check(char *pat, char *codes, int expand);
extern int assign_sm(BNST_DATA *bp, char *cp);
extern int sm_all_match(char *c, char *target);
extern int delete_specified_sm(char *sm, char *del);
extern void sm2feature(SENTENCE_DATA *sp);
extern char *check_noun_sm(char *key);

/* main.c */
extern int one_sentence_analysis(SENTENCE_DATA *sp, FILE *input);
extern void usage();

/* para_dpnd.c */
extern int _check_para_d_struct(SENTENCE_DATA *sp, int str, int end,
				int extend_p, int limit, int *s_p);
extern void init_mask_matrix(SENTENCE_DATA *sp);
extern int check_dpnd_in_para(SENTENCE_DATA *sp);

/* para_analysis.c */
extern int para_recovery(SENTENCE_DATA *sp);
extern int check_para_key(SENTENCE_DATA *sp);
extern void detect_all_para_scope(SENTENCE_DATA *sp);
extern int detect_para_scope(SENTENCE_DATA *sp, int para_num, int restrict_p);

/* para_relation.c */
extern int detect_para_relation(SENTENCE_DATA *sp);

/* para_revision.c */
extern void revise_para_rel(SENTENCE_DATA *sp, int pre, int pos);
extern void revise_para_kakari(SENTENCE_DATA *sp, int num, int *array);

/* proper.c */
extern void ne_para_analysis(SENTENCE_DATA *sp);

/* quote.c */
extern int quote(SENTENCE_DATA *sp);

/* read_data.c */
extern int imi2feature(char *str, MRPH_DATA *m_ptr);
extern int read_mrph(SENTENCE_DATA *sp, FILE *fp);
extern void change_mrph(MRPH_DATA *m_ptr, FEATURE *f);
extern void assign_mrph_feature(MrphRule *s_r_ptr, int r_size,
				MRPH_DATA *s_m_ptr, int m_length,
				int mode, int break_mode, int direction);
extern void assign_general_feature(void *data, int size, int flag);
extern int make_bunsetsu(SENTENCE_DATA *sp);
extern int make_bunsetsu_pm(SENTENCE_DATA *sp);
extern void print_mrphs(SENTENCE_DATA *sp, int flag);
extern void assign_dpnd_rule(SENTENCE_DATA *sp);
extern int calc_bnst_length(SENTENCE_DATA *sp, BNST_DATA *b_ptr);
extern void make_tag_units(SENTENCE_DATA *sp);

/* read_rule.c */
extern int case2num(char *cp);
extern char *get_rulev(char *cp);
extern void read_homo_rule(char *file_name);
extern void read_general_rule(RuleVector *rule);
extern void read_dpnd_rule(char *file_name);
extern void read_koou_rule(char *file_name);
extern void read_mrph_rule(char *file_name, MrphRule *rp, int *count, int max);
extern void read_bnst_rule(char *file_name, BnstRule *rp, int *count, int max);

/* regexp.c */
extern void store_regexpmrphs(REGEXPMRPHS **mspp, CELL *cell);
extern void store_regexpbnsts(REGEXPBNSTS **bspp, CELL *cell);
extern int regexpmrph_match(REGEXPMRPH *ptr1, MRPH_DATA *ptr2);
extern int regexpmrphrule_match(MrphRule *r_ptr, MRPH_DATA *d_ptr,
				int bw_length, int fw_length);
extern int regexpbnstrule_match(BnstRule *r_ptr, BNST_DATA *d_ptr,
				int bw_length, int fw_length);
extern int regexptagrule_match(BnstRule *r_ptr, TAG_DATA *d_ptr, 
			       int bw_length, int fw_length);
extern int _regexpbnst_match(REGEXPMRPHS *r_ptr, BNST_DATA *b_ptr);

/* thesaurus.c */
extern void init_thesaurus();
extern void close_thesaurus();
extern char *get_str_code(unsigned char *cp, int flag);
extern void get_bnst_code_all(BNST_DATA *ptr);
extern void get_bnst_code(BNST_DATA *ptr, int flag);
extern int code_depth(char *cp, int code_size);
extern float general_code_match(THESAURUS_FILE *th, char *c1, char *c2);
extern float calc_similarity(char *exd, char *exp, int expand);
extern float CalcWordsSimilarity(char *exd, char **exp, int num, int *pos);
extern float CalcSmWordsSimilarity(char *smd, char **exp, int num, int *pos, char *del, int expand);
extern void overflowed_function(char *str, int max, char *function);
extern char *get_most_similar_code(char *exd, char *exp);

/* tools.c */
extern void *malloc_data(size_t size, char *comment);
extern void *realloc_data(void *ptr, size_t size, char *comment);
extern void init_hash();
extern int hash(unsigned char *key, int keylen);
extern unsigned char *katakana2hiragana(unsigned char *cp);
extern char *strdup_with_check(const char *s);

/* tree_conv.c */
extern int make_dpnd_tree(SENTENCE_DATA *sp);
extern void init_bnst_tree_property(SENTENCE_DATA *sp);
extern BNST_DATA *t_add_node(BNST_DATA *parent, BNST_DATA *child, int pos);

extern char **GetDefinitionFromBunsetsu(BNST_DATA *bp);
extern int ParseSentence(SENTENCE_DATA *s, char *input);

/* KNP 初期化 */
extern char *Knprule_Dirname;
extern char *Knpdict_Dirname;
extern RuleVector *RULE;
extern int CurrentRuleNum;
extern int RuleNumMax;
extern char *DICT[];

extern GeneralRuleType *GeneralRuleArray;
extern int GeneralRuleNum;
extern int GeneralRuleMax;

/* Server Client extention */
extern FILE *Infp;
extern FILE *Outfp;
extern int   OptMode;

/*====================================================================
				 END
====================================================================*/
