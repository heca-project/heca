#!/usr/bin/env bash

set -e
set -o pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

res=`$DIR/../target/debug/heca --language "en_US" --print json convert --datefmt ISO 1990/1/1|jq ".[0].day"`
if [[ $res = 4 ]] ; then
	echo convert 1 test works 
else
	echo convert 1 test broken - $res
	exit 1
fi


res=`$DIR/../target/debug/heca --language "en_US" --print json convert --datefmt ISO 1990/1/1|jq ".[0].month"`
if [[ $res = "\"Teves\"" ]] ; then
	echo convert 2 test works 
else
	echo convert 2 test broken - $res
	exit 1
fi

res=`$DIR/../target/debug/heca --language "en_US" --print json convert --datefmt ISO 1990/1/1|jq ".[0].year"`
if [[ $res = "5750" ]] ; then
	echo convert 3 test works 
else
	echo convert 3 test broken - $res
	exit 1
fi

res=`$DIR/../target/debug/heca --language "en_US" --print json convert 4-teves-5750 |jq ".[0]"`
if [[ $res = "\"1989-12-31T18:00:00Z\"" ]] ; then
	echo convert 4 test works 
else
	echo convert 4 test broken - $res
	exit 1
fi

res=`$DIR/../target/debug/heca --language "en_US" --print json convert 4-teves-5750 |jq ".[1]"`
if [[ $res = "\"1990-01-01T18:00:00Z\"" ]] ; then
	echo convert 5 test works 
else
	echo convert 5 test broken - $res
	exit 1
fi

set +e
res=$($DIR/../target/debug/heca --language "en_US" --print json convert 4-adar2-5750 2> /dev/null)
if [[ $res != 0 ]] ; then
	echo convert 5 test works 
else
	echo convert 5 test broken - $res
	exit 1
fi


