
#! /usr/bin/env bash
set -e

EXAMPLES_ROOT="./examples"
RELEASE_CONFIG="release"
RELEASE_FLAGS="--release"

echo " # Build tokio Examples"
pushd "${EXAMPLES_ROOT}"
#CARGO_TARGET_DIR=./target/tokio/ cargo clean $RELEASE_FLAGS
CARGO_TARGET_DIR=./target/tokio/ cargo build $RELEASE_FLAGS --features="tokio-runtime integra8/tokio-runtime"
popd

echo " # Running tokio Examples"
pushd "${EXAMPLES_ROOT}/target/tokio/${RELEASE_CONFIG}"
./validate_examples
popd

echo " # Build async-std Examples"
pushd "${EXAMPLES_ROOT}"
#CARGO_TARGET_DIR=./target/async-std/ cargo clean $RELEASE_FLAGS
CARGO_TARGET_DIR=./target/async-std/ cargo build $RELEASE_FLAGS --features="async-std-runtime integra8/async-std-runtime"
popd

echo " # Running async-std Examples"
pushd "${EXAMPLES_ROOT}/target/async-std/${RELEASE_CONFIG}"
./validate_examples
popd

