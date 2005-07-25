#ifndef CONTEXT_H
#define CONTEXT_H

#define	LOC_PARENTV	0x000002
#define	LOC_PARENTV_MC	0x000003
#define	LOC_CHILDPV	0x000200
#define	LOC_CHILDV	0x000400
#define	LOC_PARENTNPARENTV	0x000004
#define	LOC_PARENTNPARENTV_MC	0x000005
#define	LOC_PV		0x000008
#define	LOC_PV_MC	0x000009
#define	LOC_PARENTVPARENTV	0x000010
#define	LOC_PARENTVPARENTV_MC	0x000011
#define	LOC_MC		0x002001
#define	LOC_SC		0x004000
#define	LOC_PRE_OTHERS	0x008000
#define	LOC_POST_OTHERS	0x009000
#define	LOC_S1_MC	0x012001
#define	LOC_S1_SC	0x014000
#define	LOC_S1_OTHERS	0x010000
#define	LOC_S2_MC	0x022001
#define	LOC_S2_SC	0x024000
#define	LOC_S2_OTHERS	0x020000
#define	LOC_OTHERS	0x000000
/* LOC: 17ʸ�������� */
/* ���֤ο����Ѥ������ const.h �� LOC_NUMBER �򹹿����뤳�� */

/* ȯ�å����� */
#define UTYPE_ACTION_LARGE	0x000001	// ���:��
#define UTYPE_ACTION_MIDDLE	0x000002	// ���:��
#define UTYPE_ACTION_SMALL	0x000003	// ���:��
#define UTYPE_NOTES		0x000004	// α�ջ��ࡢα�ջ��ࡦ���ġ�α�ջ��ࡦ����
#define UTYPE_FOOD_PRESENTATION	0x000005	// ���ʡ�ƻ����
#define UTYPE_FOOD_STATE	0x000006	// ��������
#define UTYPE_DEGREE		0x000007	// ����
#define UTYPE_EFFECT		0x000008	// ����
#define UTYPE_ADDITION		0x000009	// ��­
#define UTYPE_SUBSTITUTION	0x000010	// ���ز�
#define UTYPE_END		0x000011	// ��λ
#define UTYPE_OTHERS		0x000000	// ��ʸ̵�롢��ʸ̵�롦���ǡ�̵�롢����¾


char *LocationNames[] = {"PARENTV", "PARENTV_MC", "CHILDPV", "CHILDV", 
			 "PARENTNPARENTV", "PARENTNPARENTV_MC", "PV", "PV_MC", 
			 "PARENTVPARENTV", "PARENTVPARENTV_MC", "MC", "SC", 
			 "PRE_OTHERS", "POST_OTHERS", "S1_MC", "S1_SC", 
			 "S1_OTHERS", "S2_MC", "S2_SC", "S2_OTHERS", "OTHERS", ""};
int LocationNums[] = {LOC_PARENTV, LOC_PARENTV_MC, LOC_CHILDPV, LOC_CHILDV, 
		      LOC_PARENTNPARENTV, LOC_PARENTNPARENTV_MC, LOC_PV, LOC_PV_MC, 
		      LOC_PARENTVPARENTV, LOC_PARENTVPARENTV_MC, LOC_MC, LOC_SC, 
		      LOC_PRE_OTHERS, LOC_POST_OTHERS, LOC_S1_MC, LOC_S1_SC, 
		      LOC_S1_OTHERS, LOC_S2_MC, LOC_S2_SC, LOC_S2_OTHERS, LOC_OTHERS, END_M};


/* for np */
int LocationOrder[][LOC_NUMBER] = {
    {END_M},
    {LOC_CHILDPV, LOC_PARENTV_MC, LOC_PARENTV, LOC_PV_MC, LOC_PV, LOC_PARENTNPARENTV_MC, LOC_PARENTVPARENTV, 
     LOC_CHILDV, LOC_PARENTNPARENTV, LOC_PRE_OTHERS, LOC_SC, LOC_PARENTVPARENTV_MC, LOC_MC, LOC_S1_MC,
     LOC_S1_OTHERS, LOC_S1_SC, LOC_POST_OTHERS, LOC_S2_MC, 
     LOC_S2_OTHERS, LOC_S2_SC, END_M},
    {LOC_CHILDV, LOC_PRE_OTHERS, LOC_CHILDPV, LOC_PV, LOC_S1_OTHERS, LOC_PV_MC, 
     LOC_PARENTNPARENTV_MC, LOC_PARENTV, LOC_PARENTVPARENTV_MC, LOC_S1_MC, 
     LOC_PARENTVPARENTV, LOC_PARENTNPARENTV, LOC_S2_OTHERS, LOC_PARENTV_MC, 
     LOC_SC, LOC_S2_MC, LOC_POST_OTHERS, LOC_MC, LOC_S2_SC, 
     LOC_S1_SC, END_M},
    {LOC_CHILDPV, LOC_PARENTVPARENTV, LOC_PV, LOC_PRE_OTHERS, LOC_CHILDV, 
     LOC_PARENTV, LOC_S1_OTHERS, LOC_PARENTVPARENTV_MC, LOC_S1_MC, LOC_PV_MC,
     LOC_S1_SC, LOC_SC, LOC_S2_OTHERS, LOC_PARENTV_MC, LOC_PARENTNPARENTV, 
     LOC_MC, LOC_PARENTNPARENTV_MC, LOC_S2_MC, LOC_POST_OTHERS, 
     LOC_S2_SC, END_M},
};

#endif