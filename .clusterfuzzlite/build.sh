#!/bin/bash
set -euo pipefail
cd "$SRC/fuzz"
cargo fuzz build -O --sanitizer address
cp "$SRC/fuzz/target/x86_64-unknown-linux-gnu/release/stream_fuzzer" "$OUT/"
cp "$SRC/fuzz/target/x86_64-unknown-linux-gnu/release/state_fuzzer" "$OUT/"
cp "$SRC/fuzz/target/x86_64-unknown-linux-gnu/release/recovery_fuzzer" "$OUT/"
cp "$SRC/fuzz/target/x86_64-unknown-linux-gnu/release/journal_fuzzer" "$OUT/"
