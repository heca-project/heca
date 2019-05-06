#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

ErevRoshHashanah=`$DIR/../target/release/heca --language "en_US" --print json list 5750 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "1990-09-18T18:00:00Z") | .name | .CustomVal'`
if [[ $ErevRoshHashanah = '"ErevRoshHashanah"' ]] ; then
	echo convert 1 test works 
else
	echo convert 1 test broken - $ErevRoshHashanah
	exit 1
fi

Pesach1990=`$DIR/../target/release/heca --language "en_US" --print json list 5750 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "1990-04-09T18:00:00Z") | .name | .TorahReading | .YomTov '`
if [[ $Pesach1990 = '"Pesach1"' ]] ; then
	echo convert 3 test works 
else
	echo convert 3 test broken - $Pesach1990
	exit 1
fi

$DIR/../target/release/heca --language "en_US" --print json list 5750 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "1990-04-10T18:00:00Z") | .name | .CustomVal' | grep "Omer1" >/dev/null
if [[ $? -eq 0 ]] ; then
	echo convert 4 test works 
else
	echo convert 4 test broken - $?
	exit 1
fi

$DIR/../target/release/heca --language "en_US" --print json list 5750 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "1990-04-10T18:00:00Z") | .name | .TorahReading | .YomTov' | grep "Pesach2" >/dev/null
if [[ $? -eq 0 ]] ; then
	echo convert 5 test works 
else
	echo convert 5 test broken - $?
	exit 1
fi

$DIR/../target/release/heca --language "en_US" --print json list 5779 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "2019-05-03T18:00:00Z") | .name |.TorahReading | .Shabbos' |grep "AchareiMos" >/dev/null
if [[ $? -eq 0 ]] ; then
	echo convert 6 test works 
else
	echo convert 6 test broken - $?
	exit 1
fi

for i in $(seq 50 500) ; do

echo checking year $i

outputOfYear=`$DIR/../target/release/heca --language "en_US" --print json list $i --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer`

day49ofomer=`echo $outputOfYear|jq '. | .[] | select (.name.CustomVal == "Omer49") | .day'`
dayerevshavuos=`echo $outputOfYear|jq '. | .[] | select (.name.CustomVal == "ErevShavuos") | .day'`

if [[ $day49ofomer != $dayerevshavuos ]] ; then
  echo "last day of omer != erev shavuos in year $i"
  exit 1
fi

day1ofomer=`echo $outputOfYear|jq '. | .[] | select (.name.CustomVal == "Omer1") | .day'`
day2ofpesach=`echo $outputOfYear|jq '. | .[] | select (.name.TorahReading.YomTov == "Pesach2") | .day'`

if [[ $day1ofomer != $day2ofpesach ]] ; then
  echo "1st day omer != day 2 of pesach $i"
  exit 1
fi


done

echo convert 7 test works

echo checking years 1980 through 2080
for i in `seq 1980 2080`
do
echo checking year $i

OrigErevYomKippur=$(date -d @$(( $(date -d $(cat holidays_1980_9999 |grep "/$i Erev Yom Kippur" |cut -f1 -d ' ') +%s) - 86400)))
MyErevYomKippur=$(date -d @$(date -d $($DIR/../target/release/heca --language "en_US" --print regular list --no-sort --type gregorian $i --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer |grep " Erev Yom Kippur" |cut -f1 -d ' ') +%s))

if [[ $OrigErevYomKippur != $MyErevYomKippur ]] ; then

echo Erev Yom Kippur off for year $i -- $OrigErevYomKippur $MyErevYomKippur
exit 1
fi

OrigErevYomKippur=$(date -d @$(( $(date -d $(cat holidays_1980_9999 |grep "/$i Erev Yom Kippur" |cut -f1 -d ' ') +%s) - 86400)))
MyErevYomKippur=$(date -d @$(date -d $($DIR/../target/release/heca --language "en_US" --print regular list --no-sort --type gregorian $i --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer |grep " Erev Yom Kippur" |cut -f1 -d ' ') +%s))

if [[ $OrigErevYomKippur != $MyErevYomKippur ]] ; then

echo Erev Yom Kippur off for year $i -- $OrigErevYomKippur $MyErevYomKippur
exit 1
fi



OrigYomKippur=$(date -d @$(( $(date -d $(cat holidays_1980_9999 |grep "/$i Yom Kippur" |cut -f1 -d ' ') +%s) - 86400)))
MyYomKippur=$(date -d @$(date -d $($DIR/../target/release/heca --language "en_US" --print regular list --no-sort --type gregorian $i --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer |grep " Yom Kippur" |grep -v "Erev" |cut -f1 -d ' ') +%s))

if [[ $OrigYomKippur != $MyYomKippur ]] ; then

echo Yom Kippur off for year $i -- $OrigYomKippur $MyYomKippur
exit 1
fi



done
echo done checking years 1980 through 9999

ErevRoshHashanah=`$DIR/../target/release/heca --language "he_IL" --print json list 5750 --no-sort --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "1990-09-18T18:00:00Z") | .name | .CustomVal'`
if [[ $ErevRoshHashanah = '"ErevRoshHashanah"' ]] ; then
	echo convert 1 il test works 
else
	echo convert 1 il test broken - $ErevRoshHashanah
	exit 1
fi

$DIR/../target/release/heca --language "he_IL" --print json list 5779 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "2019-05-03T18:00:00Z") | .name |.TorahReading | .Shabbos' |grep "Kedoshim" >/dev/null
if [[ $? -eq 0 ]] ; then
	echo convert 6 il test works 
else
	echo convert 6 il test broken 
	exit 1
fi

echo Checking all parshas

mine=("Bereishis" "Noach" "Lech Lecha" "Vayeira" "Chayei Sarah" "Toldos" "Vayetzei" "Vayishlach" "Vayeshev" "Miketz" "Vayigash" "Vayechi" \
      "Shemos" "Vaeira" "Bo" "Beshalach" "Yisro" "Mishpatim" "Terumah" "Tetzaveh" "Ki Sisa" "Vayakhel/Pikudei" "Vayakhel" "Pikudei" \
      "Vayikra" "Tzav" "Shemini" "Tazriya/Metzorah" "Tazriya" "Metzorah" "Acharei Mos/Kedoshim" "Acharei Mos" "Kedoshim" "Emor" "Behar/Bechukosai" "Behar" "Bechukosai" \
      "Bamidbar" "Naso" "Behaaloscha" "Shlach" "Korach" "Chukas/Balak" "Chukas" "Balak" "Pinchas" "Matos/Maasei" "Matos" "Maasei" \
	  "Devarim" "Vaeschanan" "Eikev" "Re'eh" "Shoftim" "Ki Seitzei" "Ki Savo" "Nitzavim/Vayelech" "Nitzavim" "Vayelech" \
       "Parshas Zachor" "Parshas HaChodesh" "Parshas Parah" "Parshas Shekalim")
hebcal=("Bereshit" "Noach" "Lech-Lecha" "Vayera" "Chayei Sara" "Toldot" "Vayetzei" "Vayishlach" "Vayeshev" "Miketz" "Vayigash" "Vayechi" \
        "Shemot" "Vaera" "Bo" "Beshalach" "Yitro" "Mishpatim" "Terumah" "Tetzaveh" "Ki Tisa" "Vayakhel-Pekudei" "Vayakhel" "Pekudei" \
		"Vayikra" "Tzav" "Shmini" "Tazria-Metzora" "Tazria" "Metzora" "Achrei Mot-Kedoshim" "Achrei Mot" "Kedoshim" "Emor" "Behar-Bechukotai" "Behar" "Bechukotai" \
		"Bamidbar" "Nasso" "Beha'alotcha" "Sh'lach" "Korach" "Chukat-Balak" "Chukat" "Balak" "Pinchas" "Matot-Masei" "Matot" "Masei" \
		"Devarim" "Vaetchanan" "Eikev" "Re'eh" "Shoftim" "Ki Teitzei" "Ki Tavo" "Nitzavim-Vayeilech" "Nitzavim" "Vayeilech" \
		"Shabbat Zachor" "Shabbat HaChodesh" "Shabbat Parah" "Shabbat Shekalim")


for i in `seq 5745 5840`
do

  echo Processing year $i

  if [[ ${#mine[@]} != ${#hebcal[@]} ]] ; then 
    echo "Assert mine != hebcal" ${#mine[@]} ${#hebcal[@]}
    for j in `seq 0 63` 
    do

      echo "Assert mine != hebcal" ${mine[$j]} ${hebcal[$j]}

    done
  fi

  echo checking Chutz Laaretz
  for j in $(seq 0 64)
  do    
    MyFound=$($DIR/../target/release/heca --language "en_US" --print regular list --no-sort --type hebrew "$i" --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer | grep "${mine[$j]}" -m1 |cut -f1 -d ' ')
	OrigFound=$(cat parsha_50_500 |grep " $i"  | grep -v "Atzeret" | grep "${hebcal[$j]}" -m1 |cut -f1 -d ' ' )

	if [[ $MyFound != "" ]] ; then
      MyDate=$(date -d @$(date -d "$MyFound" +%s))	
      OrigDate="$(date -d @$(( $(date -d $OrigFound +%s) - 86400)))"
      if [[ $MyDate != "$OrigDate" ]] ; then
        echo "${mine[$j]} dates aren\'t equal - \"$MyDate\" \"$OrigDate\"" - $i
        exit 1
      fi              
    elif [[ $MyFound != "$OrigFound" ]] ; then 
	  echo "${mine[$j]} dates aren\'t equal - \"$MyFound\" \"$OrigFound\""
      exit 1
	fi
  done


  echo "Checking Eretz Yisrael"
  for j in $(seq 0 64)
  do    
    MyFound=$($DIR/../target/release/heca --language "en_US"  --print regular list --location Israel --no-sort --type hebrew "$i" --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer | grep "${mine[$j]}" -m1 |cut -f1 -d ' ')
	OrigFound=$(cat parsha_il_50_500 |grep " $i"  | grep -v "Atzeret" | grep "${hebcal[$j]}" -m1 |cut -f1 -d ' ' )

	if [[ $MyFound != "" ]] ; then
      MyDate=$(date -d @$(date -d "$MyFound" +%s))	
      OrigDate="$(date -d @$(( $(date -d $OrigFound +%s) - 86400)))"
      if [[ $MyDate != "$OrigDate" ]] ; then
        echo "${mine[$j]} dates aren\'t equal - \"$MyDate\" \"$OrigDate\"" - $i
        exit 1
      fi              
    elif [[ $MyFound != "$OrigFound" ]] ; then 
	  echo "${mine[$j]} dates aren\'t equal - \"$MyFound\" \"$OrigFound\""
      exit 1
	fi
  done

done


echo done checking all Parshas