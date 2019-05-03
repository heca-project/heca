#!/usr/bin/env bash

set -e
set -o pipefail

cargo build --release 
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
$DIR/convert_test.sh
$DIR/list_test.sh

