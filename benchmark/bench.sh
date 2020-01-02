#!/bin/sh
for i in $( ps -e | awk ' { print $1 }' ); do
	sudo taskset -acp 0-32 $i 2>/dev/null >/dev/null
done

cargo build --release --target-dir=/tmp/heca >/dev/null

set +e
for i in $( ps -e | awk ' { print $1 }' ); do
	sudo taskset -acp 0 $i 2>/dev/null >/dev/null
done
set -e
~/.cargo/bin/hyperfine 'taskset -ac {cpu_count} /tmp/heca/release/heca --print=regular list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas --no-sort' --parameter-list cpu_count 1,0-32 --warmup 5 --export-markdown /tmp/1.md -u second
~/.cargo/bin/hyperfine 'taskset -ac {cpu_count} /tmp/heca/release/heca --print=regular list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas' --parameter-list cpu_count 1,0-32 --warmup 5 --export-markdown /tmp/2.md -u millisecond
~/.cargo/bin/hyperfine 'taskset -ac {cpu_count} /tmp/heca/release/heca --print=json list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas --no-sort' --parameter-list cpu_count 1,0-32 --warmup 5 --export-markdown /tmp/3.md -u millisecond
~/.cargo/bin/hyperfine 'taskset -ac {cpu_count} /tmp/heca/release/heca --print=json list 3766 --years 17000 --show yom-tov,minor-holidays,chol,special-parshas' --parameter-list cpu_count 1,0-32 --warmup 5 --export-markdown /tmp/4.md -u millisecond
~/.cargo/bin/hyperfine 'taskset -ac 1 hebcal 3766 --years 17000' --warmup 5 --export-markdown /tmp/5.md -u millisecond
~/.cargo/bin/hyperfine 'taskset -ac 0-32 hebcal 3766 --years 17000' --warmup 5 --export-markdown /tmp/6.md -u millisecond


set +e
for i in $( ps -e | awk ' { print $1 }' ); do
	sudo taskset -acp 0-32 $i 2>/dev/null >/dev/null
done

