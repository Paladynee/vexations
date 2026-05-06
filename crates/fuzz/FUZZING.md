# Fuzzing

Run the lexer fuzzer from the project root with

- `cd crates/fuzz && cargo fuzz run fuzz_lexer` or
- `cargo fuzz -C crates/fuzz run fuzz_lexer`.

The fuzzer generates random ASCII inputs and feeds them to the lexer, looking
for crashes and building a corpus of interesting test cases.

Results are saved to

- `corpus/` (for coverage) and
- `artifacts/` (for crashes/slow inputs).

Use `cargo fuzz run fuzz_lexer -- -max_len=1000 -timeout=10` to set limits, or
`clean-fuzz` from the root to reset.
