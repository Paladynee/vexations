#!/usr/bin/env bash

set -x

rm -i vexations-*.s

RUSTFLAGS='-Awarnings' cargo \
    clean -q
RUSTFLAGS='-Awarnings' cargo \
    rustc -q --release -p vexations-compiler -- \
    --emit=asm -C link-dead-code "$@"

for thing in $(find target/*/deps -type f -name '*.s'); do
    echo "$thing"
    cp --update=none-fail "$thing" . 
done
