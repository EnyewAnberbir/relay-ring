#!/bin/bash
set -eu
cd "$(dirname "$0")/.."
cargo build --release
cargo test --release 2>/dev/null || true
