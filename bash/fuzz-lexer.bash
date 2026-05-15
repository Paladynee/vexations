#!/usr/bin/env bash

cd fuzz/lexer || exit 1

RUSTFLAGS='-Ctarget-cpu=native -Awarnings' cargo afl build --release || exit 1
RUSTFLAGS='-Ctarget-cpu=native -Awarnings' cargo afl fuzz \
    -i fuzz_inputs \
    -o fuzz_outputs \
    ../target/release/lexer
