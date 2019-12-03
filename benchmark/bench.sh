#!/bin/sh

cargo build --release --target-dir=/tmp/heca >/dev/null

set -e
for i in `seq 1 5`; do
	/tmp/heca/release/heca list 3766 --years 1 --show yom-tov,minor-holidays,chol,special-parshas >/dev/null
done

TIMEFORMAT=%R
final_val=0

set +e
for i in $( ps -e | awk ' { print $1 }' ); do
	sudo taskset -acp 0 $i 2>/dev/null >/dev/null
done

set -e

final_val=0
for i in `seq 1 5`; do
	full_val=$( { time taskset -ac 1-3 /tmp/heca/release/heca list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas --no-sort >/dev/null ; } 2>&1 )
	final_val=$(awk "BEGIN {print $final_val+$full_val; exit}")
done
echo "heca  | multithreaded   | unsorted   | $final_val"

set +e
for i in $( ps -e | awk ' { print $1 }' ); do
	sudo taskset -acp 0 $i 2>/dev/null >/dev/null
done

set -e

final_val=0
for i in `seq 1 5`; do
	full_val=$( { time taskset -ac 1-3 /tmp/heca/release/heca list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas >/dev/null ; } 2>&1 )
	final_val=$(awk "BEGIN {print $final_val+$full_val; exit}")
done
echo "heca  | multithreaded   | sorted     | $final_val"



set +e
for i in $( ps -e | awk ' { print $1 }' ); do
	sudo taskset -acp 0 $i 2>/dev/null >/dev/null
done

set -e

final_val=0
for i in `seq 1 5`; do
	full_val=$( { time taskset -ac 1 /tmp/heca/release/heca list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas --no-sort >/dev/null ; } 2>&1 )
	final_val=$(awk "BEGIN {print $final_val+$full_val; exit}")
done
echo "heca  | singlethreaded  | unsorted   | $final_val"

final_val=0
for i in `seq 1 5`; do
	full_val=$( { time taskset -ac 1 /tmp/heca/release/heca list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas >/dev/null ; } 2>&1 )
	final_val=$(awk "BEGIN {print $final_val+$full_val; exit}")
done
echo "heca  | singlethreaded  | sorted     | $final_val"



for i in `seq 1 5`; do
	hebcal 3766 --years 17000 >/dev/null
done

set +e
for i in $( ps -e | awk ' { print $1 }' ); do
	sudo taskset -acp 0 $i 2>/dev/null >/dev/null
done
set -e
final_val=0
for i in `seq 1 5`; do
	full_val=$( { time taskset -ac 1-3 hebcal 3766 --years 17000 >/dev/null ; } 2>&1 )
	final_val=$(awk "BEGIN {print $final_val+$full_val; exit}")
done
echo "hebcal                               | $final_val"

set +e
for i in $( ps -e | awk ' { print $1 }' ); do
	sudo taskset -acp 0-3 $i 2>/dev/null >/dev/null
done

