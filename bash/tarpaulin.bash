#!/usr/bin/env bash
cargo tarpaulin \
    --workspace \
    --out Html \
    --output-dir coverage \
    --timeout 300 \
    --exclude-files tests examples crates/fuzz_targets crates/bench-targets \
    --all-features
