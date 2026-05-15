#!/usr/bin/env bash

set -x

rm -i vexations-*.ll

RUSTFLAGS='-Awarnings' cargo \
    clean -q
RUSTFLAGS='-Awarnings' cargo \
    rustc -q --release -p vexations-compiler -- \
    --emit=llvm-ir -C link-dead-code "$@"

for thing in $(find target/*/deps -type f -name '*.ll'); do
    echo "$thing"
    cp --update=none-fail "$thing" . 
done
