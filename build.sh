#!/bin/bash
set -eu
cd "$SRC"
if [ -d fuzz ]; then
  cd fuzz
fi
cargo fuzz build -O --sanitizer address
for t in journal_fuzzer stream_fuzzer state_fuzzer recovery_fuzzer; do
  BIN=$(find target -type f -name "$t" -path "*/release/$t" | head -n1)
  if [ -z "$BIN" ]; then
    echo "missing $t" >&2
    exit 1
  fi
  cp "$BIN" "$OUT/$t"
done
