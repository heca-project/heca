#!/bin/sh

set -e
cargo build --target "$TARGET" --release --target-dir=/tmp/heca

set +e
cargo publish

set -e
strip "/tmp/heca/$TARGET/release/heca"
name="heca-${TRAVIS_TAG}-${TARGET}"
mkdir -p /tmp/heca-staging
cp "/tmp/heca/$TARGET/release/heca" "/tmp/heca-staging/$name"
xz "/tmp/heca-staging/$name"
rm "/tmp/heca-staging/$name"
cp "/tmp/heca/$TARGET/release/heca" "/tmp/heca-staging/$name"
