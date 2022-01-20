
#! /usr/bin/env bash
set -e

# Root of workspace-like directories.
PROJECT_ROOT="."
EXAMPLES_ROOT="./examples"

echo "Build Examples"
pushd "${EXAMPLES_ROOT}"
#cargo clean
CARGO_TARGET_DIR=./target/tokio/ cargo build --features="tokio-runtime integra8/tokio-runtime"
CARGO_TARGET_DIR=./target/async-std/ cargo build --features="async-std-runtime integra8/async-std-runtime"
popd

echo "Running tokio Examples"
pushd "${EXAMPLES_ROOT}/target/tokio/debug"
./validate_examples
popd

echo "Running async-std Examples"
pushd "${EXAMPLES_ROOT}/target/async-std/debug"
./validate_examples
popd




