#!/usr/bin/env perl

use encoding 'euc-jp', STDIN => 'shiftjis';

# 分類語彙表データの整形
#
#   1. フロッピーディスク版(sakuin.dat)とモニター版(sakuin)の一行目で識別し
#      双方に対応．
#
#   2. モニター版の種々のバグをこの中で修正．
#
#   3. モニター版には英字の段落下位分類があるがこれは無視．
#
#   4. モニター版では段落内番号が行番号と行内語番号にわかれており，それぞれ
#      1桁または2桁である．
#      フロッピーディスク版(段落内番号最大3桁)とあわすためにモニター版でも
#      行番号と行内語番号をまとめて一つの数字とする．
#      幸いまとめたときに4桁になるものはない．ただし単純な連結なので一意性は
#      保証されない．

#
#  増補改訂版の一行目
#

$ZohoFirstLine = '000001,00001,A,体,関係,事柄,事柄,1.1000,01,01,01,者（もの）,者,もの,のも';

#
#  フロッピー版の一行目
#

$FloppyFirstLine = 'あ,あ,4.310,1,10,*,';

#
# モニター版(sakuin)のバグ修正用
#

$bug_fix{"いらっしゃる,いらっしゃる,2.1527,B,1,3"} = "いらっしゃる,いらっしゃる,2.1527,5B,1,3";
$bug_fix{"おいでになる,おいでになる,2.1527,B,1,2"} = "おいでになる,おいでになる,2.1527,5B,1,2";
$bug_fix{"お成りになる,おなりになる,2.1527,B,1,4"} = "お成りになる,おなりになる,2.1527,5B,1,4";
$bug_fix{"還啓,かんけい,2.1527,B,2,6"} = "還啓,かんけい,2.1527,5B,2,6";
$bug_fix{"還幸,かんこう,2.1527,B,2,5"} = "還幸,かんこう,2.1527,5B,2,5";
$bug_fix{"行啓,ぎょうけい,2.1527,B,2,4"} = "行啓,ぎょうけい,2.1527,5B,2,4";
$bug_fix{"行幸,ぎょうこう,2.1527,B,2,3"} = "行幸,ぎょうこう,2.1527,5B,2,3";
$bug_fix{"御幸,ごこう,2.1527,B,2,2"} = "御幸,ごこう,2.1527,5B,2,2";
$bug_fix{"巡幸,じゅんこう,2.1527,B,2,7"} = "巡幸,じゅんこう,2.1527,5B,2,7";
$bug_fix{"渡御,とぎょ,2.1527,B,2,1"} = "渡御,とぎょ,2.1527,5B,2,1";
$bug_fix{"遭難する,そうなんする,2.3310,B,1,4"} = "遭難する,そうなんする,2.3310,2B,1,4";
$bug_fix{"被災する,ひさいする,2.3310,B,1,2"} = "被災する,ひさいする,2.3310,2B,1,2";
$bug_fix{"罹災する,りさいする,2.3310,B,1,3"} = "罹災する,りさいする,2.3310,2B,1,3";
$bug_fix{"難儀する,なんぎする,2.3310,B,2,2"} = "難儀する,なんぎする,2.3310,2B,2,2";
$bug_fix{"避難する,ひなんする,2.3310,B,2,1"} = "避難する,ひなんする,2.3310,2B,2,1";
$bug_fix{"再任する,さいにんする,2.3312,B,1,4"} = "再任する,さいにんする,2.3312,6B,1,4";
$bug_fix{"復員する,ふくいんする,2.3312,B,1,2"} = "復員する,ふくいんする,2.3312,6B,1,2";
$bug_fix{"復職する,ふくしょくする,2.3312,B,1,3"} = "復職する,ふくしょくする,2.3312,6B,1,3";
$bug_fix{"培養する,ばいようする,2.3810,B,1,3"} = "培養する,ばいようする,2.3810,8B,1,3";
$bug_fix{"養う,やしなう,2.3810,B,1,2"} = "養う,やしなう,2.3810,8B,1,2";
$bug_fix{"養殖する,ようしょくする,2.3810,B,1,4"} = "養殖する,ようしょくする,2.3810,8B,1,4";
$bug_fix{"飼い慣らす,かいならす,2.3810,B,2,2"} = "飼い慣らす,かいならす,2.3810,8B,2,2";
$bug_fix{"飼う,かう,2.3810,B,2,1"} = "飼う,かう,2.3810,8B,2,1";
$bug_fix{"飼育する,しいくする,2.3810,B,3,2"} = "飼育する,しいくする,2.3810,8B,3,2";
$bug_fix{"飼養する,しようする,2.3810,B,3,1"} = "飼養する,しようする,2.3810,8B,3,1";
$bug_fix{"肥育する,ひいくする,2.3810,B,3,3"} = "肥育する,ひいくする,2.3810,8B,3,3";
$bug_fix{"放牧する,ほうぼくする,2.3810,B,4,1"} = "放牧する,ほうぼくする,2.3810,8B,4,1";
$bug_fix{"牧羊する,ぼくようする,2.3810,B,4,3"} = "牧羊する,ぼくようする,2.3810,8B,4,3";
$bug_fix{"遊牧する,ゆうぼくする,2.3810,B,4,2"} = "遊牧する,ゆうぼくする,2.3810,8B,4,2";
$bug_fix{"搾乳する,さくにゅうする,2.3810,B,5,1"} = "搾乳する,さくにゅうする,2.3810,8B,5,1";
$bug_fix{"官僚,かんりょぅ かんりょう,1.2330,4,1,5"} = "官僚,かんりょう,1.2330,4,1,5";
$bug_fix{"官僚,かんりょぅ かんりょう,1.2411,1,6,1"} = "官僚,かんりょう,1.2411,1,6,1";
$bug_fix{"どうして,どうして,4.318 ,1,1,6"} = "どうして,どうして,4.3180,1,1,6";
$bug_fix{"なぜ,なぜ,4.318 ,1,1,2"} = "なぜ,なぜ,4.3180,1,1,2";
$bug_fix{"何ゆえ,なにゆえ,4.318 ,1,1,4"} = "何ゆえ,なにゆえ,4.3180,1,1,4";
$bug_fix{"なんだって,なんだって,4.318 ,1,1,5"} = "なんだって,なんだって,4.3180,1,1,5";
$bug_fix{"なんで,なんで,4.318 ,1,1,3"} = "なんで,なんで,4.3180,1,1,3";
$bug_fix{"いずくんぞ,いずくんぞ,4.318 ,1,3,2"} = "いずくんぞ,いずくんぞ,4.3180,1,3,2";
$bug_fix{"なんぞ,なんぞ,4.318 ,1,3,1"} = "なんぞ,なんぞ,4.3180,1,3,1";
$bug_fix{"者,もの,1.1000,7,1,2"} = "者,-,1.1000,7,1,2"; # 読みを登録しない
$bug_fix{"者,もの,1.2020,1,1,4"} = "者,-,1.2020,1,1,4"; # 読みを登録しない
$bug_fix{"すの子��,すのこ,1.4430,18,2,1"} = "すの子,すのこ,1.4430,18,2,1";

#
# main : 最初の一行でバージョンを判断し，あとは関数を呼ぶだけ
#

$_ = <STDIN>;
chomp;
s/\r//g;

if ($_ eq $ZohoFirstLine) {
    print STDERR "Your BGH is Zoho Kaitei Version.\n";
    &zoho_format($_);
    while ( <STDIN> ) {
	chomp;
	s/\r//g;
	&zoho_format($_);
    }
}elsif ($_ eq $FloppyFirstLine) {
    print STDERR "Your BGH is Floppy Disk Version.\n";
    &fd_format($_);
    while ( <STDIN> ) {
	chomp;
	s/\r//g;
	&fd_format($_);
    }
} else {
    print STDERR "Your BGH is Monitor Version.\n";
    &monitor_format($_);
    while ( <STDIN> ) {
	chomp;
	s/\r//g;
	&monitor_format($_);
    }
}

#
# zoho_format : 増補改訂版(bunruidb.txt)の整形
# 11桁のコードを出力します
#

sub zoho_format {
    
    my ($input) = @_;

    my ($a, $b, $c, $d, $e, $f, $g, $code1, $code2, $code3, $code4, $code5, $h, $hyouki, $yomi) = split(/,|\./, $input);

    if ($hyouki =~ /＊|〓|（|\)/) {next;} # invalid items

    $code = $code1.$code2.$code3.$code4.$code5;

    foreach $item (split(/・/,$hyouki)) {
	foreach $yomi_item (split(/・/,$yomi)) {
	    print "$item/$yomi_item $code\n";
	}
    }
}

#
# fd_format : フロッピーディスク版(sakuin.dat)の整形
#

sub fd_format {
    
    my ($input) = @_;

    ($yomi, $hyouki, $code1, $code2, $code3, $code4) = split(/,|\./, $input);

    if ($hyouki =~ /（|\)/) {next;} # invalid items

    if (length($code2) == 2) {
	$code2 .= "00";		# 分類番号は後ろに0をつめる
    }
    elsif (length($code2) == 3) {
	$code2 .= "0";		# 分類番号は後ろに0をつめる
    }
    $code = sprintf("%d%d%02d%03d", $code1, $code2, $code3, $code4);

    print "$hyouki $code\n";

    if ($yomi !~ /$hyouki/ && length($yomi) > 1) {
	print "$yomi $code\n";
    }
}

#
# monitor_format : モニター版(sakuin)の整形
#

sub monitor_format {
    
    my ($input) = @_;

    if ($bug_fix{$input}) {$input = $bug_fix{$input};}

    ($hyouki, $yomi, $code1, $code2, $code3, $code4, $code5) 
	= split(/,|\./, $input);
    
    $hyouki =~ s/（[^\）]*）//g;
    $yomi =~ s/（[^\）]*）//g;
    $code3 =~ s/[A-Z]$//;
    $code = sprintf("%d%d%02d%02d%02d", $code1, $code2, $code3, $code4, $code5);

    foreach $item (split(/・/,$hyouki)) {
	print "$item $code\n";
	# 「反作用・反動が生ずる」，「なすりあう・とりあう・愛し合う」など
	# では不適当だがあきらめる．
    }	
    if ($yomi !~ /$hyouki/ && length($yomi) > 1) {
	foreach $item (split(/・/,$yomi)) {
	    print "$item $code\n";
	}
    }
}
