
#! /usr/bin/env bash
set -e


# Root of workspace-like directories.
PROJECT_ROOT="."
EXAMPLES_ROOT="./examples"

echo "Build Examples"
pushd "${EXAMPLES_ROOT}"
#cargo clean
cargo build --release
popd

echo "Running Examples"
pushd "${EXAMPLES_ROOT}/target/release"

./simple_test_with_tokio
popd




