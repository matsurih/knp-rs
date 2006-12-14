/*====================================================================

			      Base Phrase

                                               K.Yu        2006.10.23

====================================================================*/
#include "knp.h"

int 	np_matrix[BNST_MAX];
int     pp_matrix[BNST_MAX];

/*==================================================================*/
		  void init_phrase(SENTENCE_DATA *sp)
/*==================================================================*/
{
    int i;

    for (i = 0; i < sp->Bnst_num; i++) {
	np_matrix[i] = -1;
	pp_matrix[i] = -1;
    }
}

/*==================================================================*/
		  int check_phrase(SENTENCE_DATA *sp)
/*==================================================================*/
{
    int		i, j, flag;

    flag = FALSE;

    for (i = 0; i < sp->Bnst_num; i++) {
	if (check_feature((sp->bnst_data+i)->f, "NP_B")) {
	    j = i + 1;
	    while (j < sp->Bnst_num && !check_feature((sp->bnst_data+j)->f, "NP_B") && !check_feature((sp->bnst_data+j)->f, "NP_O")) {
		j++;
	    }
	    np_matrix[i] = j - 1;
	    if (OptDisplay == OPT_DEBUG) { 
		printf("NP (%d-%d)\n", i, j - 1);
	    }
	    i = j - 1;
	    flag = TRUE;
	}
    }

    for (i = 0; i < sp->Bnst_num; i++) {
	if (check_feature((sp->bnst_data+i)->f, "PP_B")) {
	    j = i + 1;
	    while (j < sp->Bnst_num && !check_feature((sp->bnst_data+j)->f, "PP_B") && !check_feature((sp->bnst_data+j)->f, "PP_O")) {
		j++;
	    }
	    pp_matrix[i] = j - 1;
	    if (OptDisplay == OPT_DEBUG) { 
		printf("PP (%d-%d)\n", i, j - 1);
	    }
	    i = j - 1;
	    flag = TRUE;
	}
    }

    return flag;
}

/*==================================================================*/
		void change_matrix_for_phrase(SENTENCE_DATA *sp)
/*==================================================================*/
{
    int i, j, k;

    for (i = 0; i < sp->Bnst_num; i++){
	if (np_matrix[i] == -1 && pp_matrix[i] == -1) {
	    continue;
	}
	else {
	    /* the head of np must be the last word */
	    if (np_matrix[i] != -1) {
		/* mask upper dpnd */
		for (j = 0; j < i; j++) {
		    for (k = i; k < np_matrix[i]; k++) {
			Dpnd_matrix[j][k] = 0;
		    }
		}
		/* mask right dpnd */
		for (j = i; j < np_matrix[i]; j++) {
		    for (k = np_matrix[i] + 1; k < sp->Bnst_num; k++) {
			Dpnd_matrix[j][k] = 0;
		    }
		}
	    }
	    /* the head of np must be the first word */
	    if (pp_matrix[i] != -1) {
		/* mask upper dpnd */
		for (j = 0; j < i; j++) {
		    for (k = i + 1; k <= pp_matrix[i]; k++) {
			Dpnd_matrix[j][k] = 0;
		    }
		}
		/* mask right dpnd */
		for (j = i + 1; j <= pp_matrix[i]; j++) {
		    for (k = pp_matrix[i]; k < sp->Bnst_num; k++) {
			Dpnd_matrix[j][k] = 0;
		    }
		}
	    }
	}
    }
}

/*==================================================================*/
		void change_matrix_for_fragment(SENTENCE_DATA *sp)
/*==================================================================*/
{
    int i, j, head_pos = -1;

    /* get the head position of this fragment, i.e. the last non-pu word */
    for (i = sp->Bnst_num - 1; i >= 0; i--){
	if (!(check_feature(sp->bnst_data[i].f, "PU"))) {
	    head_pos = i;
	    break;
	}
    }

    /* change dpnd matrix, make all the words depend on the head word */
    if (head_pos != -1) {
	for (i = 0; i < sp->Bnst_num; i++) {
	    for (j = i + 1; j < sp->Bnst_num; j++) {
		if (i != head_pos && j != head_pos) {
		    Dpnd_matrix[i][j] = 0;
		}
	    }
	}
    }
}

/*==================================================================*/
		     int base_phrase(SENTENCE_DATA *sp)
/*==================================================================*/
{
    int flag;

    init_phrase(sp);

    /* 呼応のチェック */
    flag = (check_phrase(sp) == TRUE) ? TRUE: FALSE;

    /* 行列の書き換え */
    change_matrix_for_phrase(sp); 

    return flag;
}

/*==================================================================*/
		     int fragment(SENTENCE_DATA *sp)
/*==================================================================*/
{
    int flag = 1, i;

    for (i = 0; i < sp->Bnst_num; i++) {
	if (check_feature(sp->bnst_data[i].f, "VV") ||
	    check_feature(sp->bnst_data[i].f, "VA") ||
	    check_feature(sp->bnst_data[i].f, "VC") ||
	    check_feature(sp->bnst_data[i].f, "VE")) {
	    flag = 0;
	    break;
	}
    }

    if (flag) {
	change_matrix_for_fragment(sp);
    }

    return flag;
}

/*====================================================================
                               END
====================================================================*/
