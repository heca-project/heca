#!/usr/bin/env bash

set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

ErevRoshHashanah=`$DIR/../target/release/heca --language "en_US" --print json list 5750 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "1990-09-18T18:00:00Z") | .name '`
if [[ $ErevRoshHashanah = '"ErevRoshHashanah"' ]] ; then
	echo list 1 test works 
else
	echo list 1 test broken - $ErevRoshHashanah
	exit 1
fi

Pesach1990=`$DIR/../target/release/heca --language "en_US" --print json list 5750 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "1990-04-09T18:00:00Z") | .name '`
if [[ $Pesach1990 = '"Pesach1"' ]] ; then
	echo list 2 test works 
else
	echo list 2 test broken - $Pesach1990
	exit 1
fi

$DIR/../target/release/heca --language "en_US" --print json list 5750 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "1990-04-10T18:00:00Z") | select(.type == "CustomVal") | .name' | grep "Omer1" >/dev/null
if [[ $? -eq 0 ]] ; then
	echo list 4 test works 
else
	echo list 4 test broken - $?
	exit 1
fi

$DIR/../target/release/heca --language "en_US" --print json list 5750 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "1990-04-10T18:00:00Z") | select(.type == "YomTov") | .name' >/dev/null
if [[ $? -eq 0 ]] ; then
	echo list 5 test works 
else
	echo list 5 test broken - $?
	exit 1
fi

$DIR/../target/release/heca --language "en_US" --print json list 5779 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq  '. | .[] | select(.day == "2019-05-03T18:00:00Z") | select(.type == "Shabbos")' >/dev/null
if [[ $? -eq 0 ]] ; then
	echo list 6 test works 
else
	echo list 6 test broken - $?
	exit 1
fi

for i in $(seq 50 2999; seq 3800 6000) ; do
outputOfYear=`$DIR/../target/release/heca --language "en_US" --print json list $i --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer`

day49ofomer=`echo $outputOfYear|jq '. | .[] | select(.type == "CustomVal") | select (.name == "Omer49") | .day'`
dayerevshavuos=`echo $outputOfYear|jq '. | .[] | select(.type == "CustomVal") | select (.name == "ErevShavuos") | .day'`

if [[ $day49ofomer != $dayerevshavuos ]] ; then
  echo "last day of omer != erev shavuos in year $i"
  exit 1
fi

day1ofomer=`echo $outputOfYear|jq '. | .[] | select(.type == "CustomVal") | select (.name == "Omer1") | .day'`
day2ofpesach=`echo $outputOfYear|jq '. | .[] | select(.type == "YomTov") | select (.name == "Pesach2") | .day'`

if [[ $day1ofomer != $day2ofpesach ]] ; then
  echo "1st day omer != day 2 of pesach $i"
  exit 1
fi


done

echo list 7 test works

ErevRoshHashanah=`$DIR/../target/release/heca --language "he_IL" --print json list 5750 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "1990-09-18T18:00:00Z") | select(.type == "CustomVal") | .name '`
if [[ $ErevRoshHashanah = '"ErevRoshHashanah"' ]] ; then
	echo list 1 il test works 
else
	echo list 1 il test broken - $ErevRoshHashanah
	exit 1
fi

$DIR/../target/release/heca --language "he_IL" --print json list 5779 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer|jq '. | .[] | select(.day == "2019-05-03T18:00:00Z") | select(.type = "Shabbos") | .name' |grep "Kedoshim" >/dev/null
if [[ $? -eq 0 ]] ; then
	echo list 6 il test works 
else
	echo list 6 il test broken 
	exit 1
fi
