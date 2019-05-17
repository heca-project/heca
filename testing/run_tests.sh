#!/usr/bin/env bash

set -e
set -o pipefail
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd $DIR/../lib/heca-lib
cargo test
cd $DIR/../
cargo test
