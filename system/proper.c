/*====================================================================

			     固有名詞処理

                                               S.Kurohashi 96. 7. 4

    $Id$
====================================================================*/
#include "knp.h"

DBM_FILE	proper_db = NULL, properc_db = NULL, propercase_db = NULL;
int		PROPERExist = 0;

char *CaseList[] = {"カラ格", "ガ格", "デ格", "ト格", "ニ格", "ノ格", 
		    "ヘ格", "マデ格", "ヨリ格", "ヲ格", "体言NONE", 
		    "中断線", "同格未格", "同格連体", "同格連用", 
		    "文末", "未格", "無格", "用言強NONE", "隣接", 
		    "連格", "連体", "連用", ""};

PreservedNamedEntity *pNE = NULL;

char *TableNE[] = {"人名", "地名", "組織名", "固有名詞", ""};

/*==================================================================*/
			   void init_proper()
/*==================================================================*/
{
    if ((proper_db = DBM_open(PROPER_DB_NAME, O_RDONLY, 0)) == NULL || 
	(properc_db = DBM_open(PROPERC_DB_NAME, O_RDONLY, 0)) == NULL || 
	(propercase_db = DBM_open(PROPERCASE_DB_NAME, O_RDONLY, 0)) == NULL) {
	PROPERExist = FALSE;
    } else {
	PROPERExist = TRUE;
    }
}

/*==================================================================*/
                    void close_proper()
/*==================================================================*/
{
    if (PROPERExist == TRUE) {
	DBM_close(proper_db);
	DBM_close(properc_db);
	DBM_close(propercase_db);
    }
}

/*==================================================================*/
                    char *get_proper(char *cp, DBM_FILE db)
/*==================================================================*/
{
    if (PROPERExist == FALSE) {
	cont_str[0] = '\0';
	return cont_str;
    }

    key.dptr = cp;
    if ((key.dsize = strlen(cp)) >= DBM_KEY_MAX) {
	fprintf(stderr, "Too long key <%s>.\n", key.dptr);
	cont_str[0] = '\0';
	return cont_str;
    }  
    
    content = DBM_fetch(db, key);
    if (content.dptr) {
	if (content.dsize > DBM_CON_MAX) {
	    fprintf(stderr, "Too long SM content <%.*s>.\n", content.dsize, content.dptr);
	    content.dsize = DBM_CON_MAX;
	}
	strncpy(cont_str, content.dptr, content.dsize);
	cont_str[content.dsize] = '\0';
#ifdef	GDBM
	free(content.dptr);
	content.dsize = 0;
#endif
    }
    else {
	cont_str[0] = '\0';
    }

    return cont_str;
}

/*==================================================================*/
		   void _init_NE(struct _pos_s *p)
/*==================================================================*/
{
    p->Location = 0;
    p->Person = 0;
    p->Organization = 0;
    p->Artifact = 0;
    p->Others = 0;
    p->Type[0] = '\0';
    p->Count = 0;
}

/*==================================================================*/
		    void init_NE(NamedEntity *np)
/*==================================================================*/
{
    _init_NE(&(np->AnoX));
    _init_NE(&(np->XnoB));
    _init_NE(&(np->XB));
    _init_NE(&(np->AX));
    _init_NE(&(np->self));
    _init_NE(&(np->selfSM));
    _init_NE(&(np->Case));
}

/*==================================================================*/
     void _store_NE(struct _pos_s *p, char *string, char *mtype)
/*==================================================================*/
{
    char *token, type[256];

    if (mtype)
	strcpy(p->Type, mtype);
    else
	p->Type[0] = '\0';

    /* スペースで切るのです */
    token = strtok(string, " ");
    while (token) {
	sscanf(token, "%[^:]", type);
	if (str_eq(type, "地名")) {
	    p->Location += atoi(token+strlen(type)+1);
	}
	else if (str_eq(type, "人名")) {
	    p->Person += atoi(token+strlen(type)+1);
	}
	else if (str_eq(type, "組織名")) {
	    p->Organization += atoi(token+strlen(type)+1);
	}
	else if (str_eq(type, "固有名詞")) {
	    p->Artifact += atoi(token+strlen(type)+1);
	}
	else if (str_eq(type, "その他")) {
	    p->Others += atoi(token+strlen(type)+1);
	}
	token = strtok(NULL, " ");
    }
}

/*==================================================================*/
		   char *check_class(MRPH_DATA *mp)
/*==================================================================*/
{
    if (check_feature(mp->f, "かな漢字"))
	return "かな漢字";
    else if (check_feature(mp->f, "カタカナ"))
	return "カタカナ";
    else if (check_feature(mp->f, "英記号"))
	return "英記号";
    else if (check_feature(mp->f, "数字"))
	return "数字";
    return NULL;
}

/*==================================================================*/
	 void store_NE(NamedEntity *np, char *feature, int i)
/*==================================================================*/
{
    char type[256], mtype[256], class[256];
    int offset;

    sscanf(feature, "%[^:]", type);

    if (str_eq(type, "AのX")) {
	offset = strlen(type)+1;
	sscanf(feature+offset, "%[^:]", class);

	if (i < Mrph_num-2 && 
	    ((check_feature(mrph_data[i].f, "自立") && 
	      mrph_data[i].Hinshi == 6) || 
	      (mrph_data[i].Hinshi == 14 && mrph_data[i].Bunrui == 2) || 
	      (mrph_data[i].Hinshi == 13 && mrph_data[i].Bunrui == 1)) && 
	    check_feature(mrph_data[i+2].f, "自立") && 
	    mrph_data[i+2].Hinshi == 6) {
	    /* "の"の後の自立語の文字種 */
	    strcpy(mtype, check_class(&(mrph_data[i+2])));
	    if (str_eq(class, mtype)) {
		offset += strlen(class)+1;
		_store_NE(&(np->AnoX), feature+offset, class);
	    }
	}
    }
    else if (str_eq(type, "XのB")) {
	offset = strlen(type)+1;
	sscanf(feature+offset, "%[^:]", class);

	if (i > 1 && 
	    ((check_feature(mrph_data[i].f, "自立") && 
	      mrph_data[i].Hinshi == 6) || 
	      (mrph_data[i].Hinshi == 14 && mrph_data[i].Bunrui == 2) || 
	      (mrph_data[i].Hinshi == 13 && mrph_data[i].Bunrui == 1)) && 
	    check_feature(mrph_data[i-2].f, "自立") && 
	    mrph_data[i-2].Hinshi == 6) {
	    /* "の"の前の自立語の文字種 */
	    strcpy(mtype, check_class(&(mrph_data[i-2])));
	    if (str_eq(class, mtype)) {
		offset += strlen(class)+1;
		_store_NE(&(np->XnoB), feature+offset, class);
	    }
	}
    }
    else if (str_eq(type, "XB")) {
	offset = strlen(type)+1;
	sscanf(feature+offset, "%[^:]", class);

	if (i > 0 && 
	    ((check_feature(mrph_data[i].f, "自立") && 
	      mrph_data[i].Hinshi == 6) || 
	      (mrph_data[i].Hinshi == 14 && mrph_data[i].Bunrui == 2) || 
	      (mrph_data[i].Hinshi == 13 && mrph_data[i].Bunrui == 1)) && 
	     (check_feature(mrph_data[i-1].f, "自立") && 
	      mrph_data[i-1].Hinshi == 6)) {
	    /* ひとつ前の自立語(または接辞)の文字種 */
	    strcpy(mtype, check_class(&(mrph_data[i-1])));
	    if (str_eq(class, mtype)) {
		offset += strlen(class)+1;
		_store_NE(&(np->XB), feature+offset, class);
	    }
	}
    }
    else if (str_eq(type, "単語")) {
	_store_NE(&(np->self), feature+strlen(type)+1, NULL);
    }
    else if (str_eq(type, "文字種")) {
	_store_NE(&(np->selfSM), feature+strlen(type)+1, NULL);
    }
    else if (str_eq(type, "AX")) {
	offset = strlen(type)+1;
	sscanf(feature+offset, "%[^:]", class);

	if (i < Mrph_num-1 && 
	    ((check_feature(mrph_data[i].f, "自立") && 
	      mrph_data[i].Hinshi == 6) || 
	      (mrph_data[i].Hinshi == 14 && mrph_data[i].Bunrui == 2) || 
	      (mrph_data[i].Hinshi == 13 && mrph_data[i].Bunrui == 1)) && 
	     (check_feature(mrph_data[i+1].f, "自立") && 
	      mrph_data[i+1].Hinshi == 6)) {
	    strcpy(mtype, check_class(&(mrph_data[i+1])));
	    if (str_eq(class, mtype)) {
		offset += strlen(class)+1;
		_store_NE(&(np->AX), feature+offset, class);
	    }
	}
    }
}

/*==================================================================*/
		  float merge_ratio(int n1, int n2)
/*==================================================================*/
{
    return (float)n1/(n1+5);
}

/*==================================================================*/
   float calculate_NE(int v1, int n1, int v2, int n2, float ratio)
/*==================================================================*/
{
    if (n1 && n2)
	return (float)v1/n1*100*ratio+(float)v2/n2*100*(1-ratio);
    else if (n1)
	return (float)v1/n1*100;
    /* return (float)v1/n1*100*ratio; */
    else if (n2)
	return (float)v2/n2*100*(1-ratio);
    return 0;
}

/*==================================================================*/
     struct _pos_s _NE2mrph(struct _pos_s *p1, struct _pos_s *p2)
/*==================================================================*/
{
    int n1, n2;
    float ratio;
    struct _pos_s r;

    _init_NE(&r);

    /* 文字種制約の伝播 */
    if (p1->Type[0])
	strcpy(r.Type, p1->Type);
    else if (p2->Type[0])
	strcpy(r.Type, p2->Type);
	

    n1 = p1->Location + p1->Person + p1->Organization + p1->Artifact + p1->Others;
    n2 = p2->Location + p2->Person + p2->Organization + p2->Artifact + p2->Others;

    r.Count = n1;

    /* 単語レベルの情報と意味素レベルの情報をマージする割り合いの計算 */
    ratio = merge_ratio(n1, n2);

    if (n1 || n2) {
	if (p1->Location || p2->Location)
	    r.Location = calculate_NE(p1->Location, n1, p2->Location, n2, ratio);
	if (p1->Person || p2->Person)
	    r.Person = calculate_NE(p1->Person, n1, p2->Person, n2, ratio);
	if (p1->Organization || p2->Organization)
	    r.Organization = calculate_NE(p1->Organization, n1, p2->Organization, n2, ratio);
	if (p1->Artifact || p2->Artifact)
	    r.Artifact = calculate_NE(p1->Artifact, n1, p2->Artifact, n2, ratio);
	if (p1->Others || p2->Others)
	    r.Others = calculate_NE(p1->Others, n1, p2->Others, n2, ratio);
    }
    return r;
}

/*==================================================================*/
	      struct _pos_s _NE2mrphS(struct _pos_s *p)
/*==================================================================*/
{
    int n;
    struct _pos_s r;

    _init_NE(&r);

    n = p->Location + p->Person + p->Organization + p->Artifact + p->Others;

    r.Count = n;

    if (n) {
	if (p->Location) {
	    r.Location = (float)p->Location/n*100;
	}
	if (p->Person) {
	    r.Person = (float)p->Person/n*100;
	}
	if (p->Organization) {
	    r.Organization = (float)p->Organization/n*100;
	}
	if (p->Artifact) {
	    r.Artifact = (float)p->Artifact/n*100;
	}
	if (p->Others) {
	    r.Others = (float)p->Others/n*100;
	}
    }
    return r;
}

/*==================================================================*/
  void NE2mrph(NamedEntity *np1, NamedEntity *np2, MRPH_DATA *mp)
/*==================================================================*/
{
    mp->NE.AnoX = _NE2mrph(&(np1->AnoX), &(np2->AnoX));
    mp->NE.XnoB = _NE2mrph(&(np1->XnoB), &(np2->XnoB));
    mp->NE.XB = _NE2mrph(&(np1->XB), &(np2->XB));
    mp->NE.AX = _NE2mrph(&(np1->AX), &(np2->AX));
    mp->NE.self = _NE2mrphS(&(np1->self));
    mp->NE.selfSM = _NE2mrphS(&(np2->selfSM));
}

/*==================================================================*/
void _NE2feature(struct _pos_s *p, MRPH_DATA *mp, char *type, int flag)
/*==================================================================*/
{
    int n, length, i, first = 0;
    char buffer[256], element[5][13];

    n = p->Location + p->Person + p->Organization + p->Artifact + p->Others;

    if (n || flag == 2) {
	for (i = 0; i < 5; i++) {
	    element[i][0] = '\0';
	}
	if (p->Location) {
	    sprintf(element[0], "地名:%d", p->Location);
	}
	if (p->Person) {
	    sprintf(element[1], "人名:%d", p->Person);
	}
	if (p->Organization) {
	    sprintf(element[2], "組織名:%d", p->Organization);
	}
	if (p->Artifact) {
	    sprintf(element[3], "固有名詞:%d", p->Artifact);
	}
	if (p->Others) {
	    sprintf(element[4], "その他:%d", p->Others);
	}

	/* length = 0;
	for (i = 0; i < 5; i++) {
	    if (element[i][0])
		length += strlen(element[i])+1;
	}
	buffer = (char *)malloc_data(strlen(type)+length+1+(int)log10(p->Count)+8, "_NE2feature"); */

	if (!flag)
	    sprintf(buffer, "%s:", type);
	else if (flag == 1)
	    sprintf(buffer, "%s:%s:", type, p->Type);
	else if (flag == 2) {
	    if (p->Count)
		sprintf(buffer, "%s%%頻度 %d:", type, p->Count);
	    else
		sprintf(buffer, "%s%%頻度 %d", type, p->Count);
	}

	for (i = 0; i < 5; i++) {
	    if (element[i][0]) {
		if (first++)
		    strcat(buffer, " ");
		strcat(buffer, element[i]);
	    }
	}

	assign_cfeature(&(mp->f), buffer);
	/* free(buffer); */
    }
}

/*==================================================================*/
		    void NE2feature(MRPH_DATA *mp)
/*==================================================================*/
{
    _NE2feature(&(mp->eNE.AnoX), mp, "AのX", 1);
    _NE2feature(&(mp->eNE.XnoB), mp, "XのB", 1);
    _NE2feature(&(mp->eNE.XB), mp, "XB", 1);
    _NE2feature(&(mp->eNE.AX), mp, "AX", 1);
    _NE2feature(&(mp->eNE.self), mp, "単語", 2);
    _NE2feature(&(mp->eNE.selfSM), mp, "文字種", 0);
    _NE2feature(&(mp->eNE.Case), mp, "格", 0);
}

/*==================================================================*/
    struct _pos_s _merge_NE(struct _pos_s *p1, struct _pos_s *p2)
/*==================================================================*/
{
    struct _pos_s p;
    int n1, n2;
    float ratio;

    n1 = p1->Location + p1->Person + p1->Organization + p1->Artifact + p1->Others;
    n2 = p2->Location + p2->Person + p2->Organization + p2->Artifact + p2->Others;
    ratio = merge_ratio(n1, n2);

    p.Location = ratio*p1->Location+(1-ratio)*p2->Location;
    p.Person = ratio*p1->Person+(1-ratio)*p2->Person;
    p.Organization = ratio*p1->Organization+(1-ratio)*p2->Organization;
    p.Artifact = ratio*p1->Artifact+(1-ratio)*p2->Artifact;
    p.Others = ratio*p1->Others+(1-ratio)*p2->Others;

    return p;
}

/*==================================================================*/
       NamedEntity merge_NE(NamedEntity *np1, NamedEntity *np2)
/*==================================================================*/
{
    NamedEntity ne;

    ne.AnoX = _merge_NE(&(np1->AnoX), &(np2->AnoX));
    ne.XnoB = _merge_NE(&(np1->XnoB), &(np2->XnoB));
    ne.XB = _merge_NE(&(np1->XB), &(np2->XB));
    ne.self = _merge_NE(&(np1->self), &(np2->self));
    ne.AX = _merge_NE(&(np1->AX), &(np2->AX));

    return ne;
}

/*==================================================================*/
		   char *get_proper_case(char *cp)
/*==================================================================*/
{
    char *dic_content, *pre_pos;

    dic_content = get_proper(cp, propercase_db);
    if (*dic_content != NULL) {
	for (cp = pre_pos = dic_content; *cp; cp++) {
	    if (*cp == '/') {
		*cp = '\0';
		/* store_NE(&ne[0], pre_pos); */
		pre_pos = cp + 1;
	    }
	}
	/* store_NE(&ne[0], pre_pos); */
    }
}

/*==================================================================*/
		 void store_NEC(char *feature, int i)
/*==================================================================*/
{
    char type[256];
    int j, offset;
    struct _pos_s ne;

    _init_NE(&ne);

    sscanf(feature, "%[^:]", type);
    offset = strlen(type)+1;

    for (j = 0; CaseList[j][0]; j++) {
	if (str_eq(type, CaseList[j])) {
	    _store_NE(&ne, feature+offset, NULL);
	    mrph_data[i].Case[j] = _NE2mrphS(&ne);
	    break;
	}
    }
}

/*==================================================================*/
		   void assign_f_from_dic(int num)
/*==================================================================*/
{
    char *dic_content, *pre_pos, *cp, *sm, *type;
    char code[13];
    int i, smn;
    NamedEntity ne[2];
    MRPH_DATA *mp;

    code[12] = '\0';

    mp = &(mrph_data[num]);

    /* 初期化 */
    init_NE(&ne[0]);
    init_NE(&ne[1]);

    /* 表記による検索 */
    dic_content = get_proper(mp->Goi, proper_db);
    if (*dic_content != NULL) {
	for (cp = pre_pos = dic_content; *cp; cp++) {
	    if (*cp == '/') {
		*cp = '\0';
		store_NE(&ne[0], pre_pos, num);
		pre_pos = cp + 1;
	    }
	}
	store_NE(&ne[0], pre_pos, num);
    }

    /* ここで入力形態素に意味素を与えておく */
    /* sm = (char *)get_sm(mp->Goi); */

    /* 意味素による検索 */
    if (OptNE == OPT_NESM && mp->SM[0]) {
	/* smn = strlen(sm);
	   strncpy(mp->SM, sm, smn);
	   smn = smn/SM_CODE_SIZE; */
	smn = strlen(mp->SM)/SM_CODE_SIZE;

	for (i = 0; i < smn; i++) {
	    code[0] = '1';
	    code[1] = '\0';
	    strncat(code, mp->SM+SM_CODE_SIZE*i+1, SM_CODE_SIZE-1);
	    dic_content = get_proper(code, properc_db);
	    if (*dic_content != NULL) {
		for (cp = pre_pos = dic_content; *cp; cp++) {
		    if (*cp == '/') {
			*cp = '\0';
			store_NE(&ne[1], pre_pos, num);
			pre_pos = cp + 1;
		    }
		}
		store_NE(&ne[1], pre_pos, num);
	    }
	}
    }

    /* 文字種の取得 */
    type = check_class(mp);

    /* 文字種による検索 */
    if (type) {
	dic_content = get_proper(type, properc_db);
	if (*dic_content != NULL) {
	    store_NE(&ne[1], dic_content, num);
	}
    }

    /* 格 */
    dic_content = get_proper(mp->Goi, propercase_db);
    if (*dic_content != NULL) {
	for (cp = pre_pos = dic_content; *cp; cp++) {
	    if (*cp == '/') {
		*cp = '\0';
		store_NEC(pre_pos, num);
		pre_pos = cp + 1;
	    }
	}
	store_NEC(pre_pos, num);
    }

    /*            ne[0]    ne[1]
	   -----------------
	   XB     表記     意味素
	   AX     表記     意味素
	   AnoX   表記     意味素
	   XnoB   表記     意味素
	   self   表記
	   selfSM          文字種 */

    NE2mrph(&ne[0], &ne[1], mp);
}

/*==================================================================*/
			  void NE_analysis()
/*==================================================================*/
{
    int i, j, k, h, pos, apos, flag = 0, match_tail;
    char decision[9], *cp;
    MrphRule *r_ptr;
    MRPH_DATA *m_ptr;
    BNST_DATA *b_ptr;
    TOTAL_MGR *tm = &Best_mgr;

    for (i = 0; i < Mrph_num; i++) {
	/* mrph_data[i].SM[0] = '\0'; */
	init_NE(&(mrph_data[i].NE));
	init_NE(&(mrph_data[i].eNE));

	for (j = 0; CaseList[j][0]; j++) {
	    _init_NE(&(mrph_data[i].Case[j]));
	}

	assign_f_from_dic(i);

	/* 単語と文字種のコピー */
	mrph_data[i].eNE.self = mrph_data[i].NE.self;
	mrph_data[i].eNE.selfSM = mrph_data[i].NE.selfSM;
    }

    /* とりあえずすべての形態素にふっておこう */
    for (i = 0; i < Mrph_num; i++) {
	/* 前の隣接語 (自立語, 名詞性名詞接尾辞, 名詞接頭辞だけ) */
	if (i > 0)
	    mrph_data[i].eNE.AX = mrph_data[i-1].NE.AX;

	/* 後の隣接語 (自立語, 名詞性名詞接尾辞, 名詞接頭辞だけ) */
	if (i < Mrph_num-1)
	    mrph_data[i].eNE.XB = mrph_data[i+1].NE.XB;

	/* A の B */
	if (flag != 2 && mrph_data[i].Hinshi == 6 && check_feature(mrph_data[i].f, "自立")) {
	    flag = 1;
	    apos = i;
	}
	else if (flag == 1 && str_eq(mrph_data[i].Goi, "の") && mrph_data[i].Hinshi == 9) {
	    flag = 2;
	}
	else if (flag == 2 && mrph_data[i].Hinshi == 6 && check_feature(mrph_data[i].f, "自立")) {
	    mrph_data[i].eNE.AnoX = mrph_data[apos].NE.AnoX;
	    mrph_data[apos].eNE.XnoB = mrph_data[i].NE.XnoB;

	    flag = 1;
	    apos = i;
	}
	else {
	    flag = 0;
	}
    }

    /* 格 */
    for (i = 0; i < Bnst_num; i++) {
	h = tm->dpnd.head[i];
	cp = (char *)check_feature(bnst_data[i].f, "係");
	if (cp) {
	    for (j = 0; CaseList[j][0]; j++) {
		if (str_eq(cp+3, CaseList[j])) {
		    bnst_data[i].mrph_ptr->eNE.Case = bnst_data[h].mrph_ptr->Case[j];
		    break;
		}
	    }
	}
    }

    /* 形態素に対するルールの適用 (eNE に対して) */
    for (i = 0; i < Mrph_num; i++) {
	m_ptr = mrph_data + i;
	/* feature へ */
	NE2feature(m_ptr);

	/* 細分類決定 */
	for (j = 0, r_ptr = NERuleArray; j < CurNERuleSize; j++, r_ptr++) {
    	    if (regexpmrphrule_match(r_ptr, m_ptr) != -1) {
		assign_feature(&(m_ptr->f), &(r_ptr->f), m_ptr);
		break;
	    }
	}
    }

    /* 複合名詞ルールの適用 */
    assign_mrph_feature(CNRuleArray, CurCNRuleSize);
}

/*==================================================================*/
		      int ReturnNEcode(char *cp)
/*==================================================================*/
{
    int i = 0;

    while (TableNE[i++][0]) {
	if (str_eq(cp, TableNE[i]))
	    return i;
    }
    return 0;
}

/*==================================================================*/
	  void allocateMRPH(PreservedNamedEntity **p, int i)
/*==================================================================*/
{
    MRPH_P **mp;

    while ((*p)->next != NULL)
	p = &((*p)->next);
    mp = &((*p)->mrph);
    while (*mp != NULL)
	mp = &((*mp)->next);
    *mp = (MRPH_P *)malloc_data(sizeof(MRPH_P));
    (*mp)->data = mrph_data[i];
    (*mp)->next = NULL;
}

/*==================================================================*/
      void allocateNE(PreservedNamedEntity **p, int code, int i)
/*==================================================================*/
{
    while (*p != NULL)
	p = &((*p)->next);
    *p = (PreservedNamedEntity *)malloc_data(sizeof(PreservedNamedEntity));
    (*p)->mrph = NULL;
    (*p)->next = NULL;
    allocateMRPH(p, i);
    (*p)->Type = code;
}

/*==================================================================*/
			  void preserveNE()
/*==================================================================*/
{
    int i, code, precode = -1;
    char *cp;

    for (i = 0; i < Mrph_num; i++) {
	if (cp = (char *)check_feature(mrph_data[i].f, "複固")) {
	    code = ReturnNEcode(cp+strlen("複固:"));
	    /* 違う種類の固有名詞になるか、固有名詞が始まったとき */
	    if (code != precode)
		allocateNE(&pNE, code, i);
	    /* 同じ種類の固有名詞 */
	    else
		allocateMRPH(&pNE, i);
	    precode = code;
	}
	else
	    precode = -1;
    }
}

/*==================================================================*/
			    void printNE()
/*==================================================================*/
{
    PreservedNamedEntity *p = pNE;
    MRPH_P *mp;

    fprintf(stdout, "<固有名詞 スタック>\n");

    while (p) {
	mp = p->mrph;
	fprintf(stdout, "%d", p->Type);
	while (mp) {
	    fprintf(stdout, " %s", mp->data.Goi2);
	    mp = mp->next;
	}
	putchar('\n');
	p = p->next;
    }
}

/*==================================================================*/
	  int check_correspond_NE(MRPH_DATA *data, char *rule)
/*==================================================================*/
{
    int code;
    PreservedNamedEntity *p = pNE;
    MRPH_P *mp;

    code = ReturnNEcode(rule);

    while (p) {
	mp = p->mrph;
	/* とりあえず、先頭の形態素だけチェックしておく */
	if (code == p->Type && str_eq(data->Goi, mp->data.Goi))
	    return TRUE;
	p = p->next;
    }
    return FALSE;
}

/*==================================================================*/
			  int assign_agent()
/*==================================================================*/
{
    int i, j, child;
    char *cp;
    char Childs[128], Case[128], SM[128];

    for (i = 0; i < Bnst_num; i++) {
	if (cp = (char *)check_feature(bnst_data[i].f, "深層格N1")) {
	    sscanf(cp, "%*[^:]:%[^:]:%[^:]:%[^:]", 
		   Childs, Case, SM);
	    child = atoi(Childs);
	    /* 自立語全てに feature を与える */
	    for (j = 0; j < bnst_data[child].jiritu_num; j++)
		assign_cfeature(&((bnst_data[child].jiritu_ptr+j)->f), "主格");
	}
    }
}

/*==================================================================*/
		      void clearMRPH(MRPH_P *p)
/*==================================================================*/
{
    MRPH_P *old;

    while (p) {
	old = p->next;
	free(p);
	p = old;
    }
}

/*==================================================================*/
			    void clearNE()
/*==================================================================*/
{
    PreservedNamedEntity *p = pNE;
    PreservedNamedEntity *old;

    pNE = NULL;

    while (p) {
	old = p->next;
	clearMRPH(p->mrph);
	free(p);
	p = old;
    }
}

/*====================================================================
                               END
====================================================================*/
