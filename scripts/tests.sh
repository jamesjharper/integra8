
#! /usr/bin/env bash
set -e

# Root of workspace-like directories.
PROJECT_ROOT="."

echo "# Running Unit tests for tokio"
CARGO_TARGET_DIR=./target/tokio/ cargo test --features="tokio-runtime"

echo "# Running Unit tests for async"
CARGO_TARGET_DIR=./target/async-std/ cargo test --features="async-std-runtime"


