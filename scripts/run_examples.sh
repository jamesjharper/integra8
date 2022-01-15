
#! /usr/bin/env bash
set -e

# Root of workspace-like directories.
PROJECT_ROOT="."
EXAMPLES_ROOT="./examples"

echo "Build Examples"
pushd "${EXAMPLES_ROOT}"
#cargo clean
cargo build
popd

echo "Running Examples"
pushd "${EXAMPLES_ROOT}/target/debug"

./basic_sample_tests
popd




