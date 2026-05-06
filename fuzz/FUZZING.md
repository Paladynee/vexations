# Lexer Fuzzing Guide

This project uses libfuzzer to fuzz the lexer for robustness and security testing.

## Quick Start

### Using cargo-fuzz (Recommended)

1. Install `cargo-fuzz`:
   ```bash
   cargo install cargo-fuzz
   ```

2. Run the fuzzer:
   ```bash
   cargo fuzz run fuzz_lexer
   ```

3. To run with a specific CPU budget:
   ```bash
   cargo fuzz run fuzz_lexer -- -max_len=1000 -timeout=10
   ```

### Using the Custom Fuzz Mode

The project also supports a custom fuzz mode via stdin:

```bash
cargo build --bin vexations
./target/debug/vexations fuzz --fuzztarget lexer < input_file
```

Or with libfuzzer directly:
```bash
cargo fuzz run fuzz_lexer -- -dict=fuzz_targets/lexer.dict
```

## What's Being Tested

The lexer fuzzer tests that:
- The lexer doesn't crash on arbitrary ASCII input
- The lexer handles malformed input gracefully
- The lexer properly handles edge cases (empty input, max lengths, etc.)
- The lexer's error recovery works correctly

## Input Constraints

The fuzzer input must be:
- **Valid ASCII**: Non-ASCII bytes are automatically filtered
- **Padded**: The lexer requires 3 zero bytes of padding at the end (handled automatically)

## Corpus and Artifacts

After running the fuzzer, you'll find:
- `fuzz_targets/corpus/fuzz_lexer/`: Test cases that maximize coverage
- `fuzz_targets/artifacts/fuzz_lexer/`: Inputs that caused crashes or slow execution

## Interpreting Results

### Coverage
The fuzzer tracks coverage and prioritizes new execution paths. Look for increasing coverage numbers in the output.

### Crashes
If a crash is found, the input is saved in the artifacts directory and printed to stderr. Use it to debug:

```bash
cargo fuzz run fuzz_lexer fuzz_targets/artifacts/fuzz_lexer/crash-example
```

### Slow Inputs
If an input takes too long (timeout), it's saved as `slow-*`. These may indicate performance issues.

## Seed Corpus

To speed up fuzzing, add interesting test cases to the corpus:

```bash
mkdir -p fuzz_targets/corpus/fuzz_lexer
# Add your test files here
echo "let x = 42;" > fuzz_targets/corpus/fuzz_lexer/simple_let
```

## Performance Tips

1. **Build with optimizations**:
   ```bash
   RUSTFLAGS="-C opt-level=3 -C lto" cargo fuzz run fuzz_lexer
   ```

2. **Increase instrumentation threads**:
   ```bash
   cargo fuzz run fuzz_lexer -- -workers=4 -jobs=4
   ```

3. **Use a dictionary for better coverage**:
   Create `fuzz_targets/lexer.dict` with keywords and patterns.

## Long-Running Fuzzing

For continuous fuzzing:

```bash
cargo fuzz run fuzz_lexer -- -max_total_time=3600  # 1 hour
cargo fuzz run fuzz_lexer -- -timeout=5  # 5 second timeout per input
```
