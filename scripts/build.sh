
#! /usr/bin/env bash
set -e

echo "# Building for tokio"
CARGO_TARGET_DIR=./target/tokio/ cargo build --features="tokio-runtime"

echo "# Building for async-std"
CARGO_TARGET_DIR=./target/async-std/ cargo build --features="async-std-runtime"


