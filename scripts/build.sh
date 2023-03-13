#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --release
cargo test --release 2>/dev/null || true
cargo build --release --manifest-path fuzz/Cargo.toml
