#!/bin/sh

set -e
cargo build --target "$TARGET" --release --target-dir=/tmp/heca

set +e
strip "/tmp/heca/$TARGET/release/heca"
set -e
name="heca-${TRAVIS_TAG}-${TARGET}"
mkdir -p /tmp/heca-staging

cp "/tmp/heca/$TARGET/release/heca"$EXTENSION "/tmp/heca-staging/$name"$EXTENSION
