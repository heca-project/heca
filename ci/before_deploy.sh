#!/bin/sh

set -e
cargo build --target "$TARGET" --release --target-dir=/tmp/heca
strip "/tmp/heca/$TARGET/release/heca"
name="${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}"
mkdir -p /tmp/heca-staging
cp "/tmp/heca/$TARGET/release/heca" "/tmp/heca-staging/$name"
xz "/tmp/heca/$TARGET/release/heca"
cp "/tmp/heca/$TARGET/release/heca.xz" "/tmp/heca-staging/$name.xz"
