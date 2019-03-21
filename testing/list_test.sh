#!/usr/bin/env bash

set -e
set -o pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

res=`$DIR/../target/debug/heca --language "en_US" --print json list 5750 --show=yom-tov,shabbos,special-parshas,chol,minor-holidays,omer`
if [[ $res = 4 ]] ; then
	echo convert 1 test works 
else
	echo convert 1 test broken - $res
	exit 1
fi


