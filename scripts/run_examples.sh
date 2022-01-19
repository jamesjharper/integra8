
#! /usr/bin/env bash
set -e

# Root of workspace-like directories.
PROJECT_ROOT="."
EXAMPLES_ROOT="./examples"

echo "Build Examples"
pushd "${EXAMPLES_ROOT}"
#cargo clean
#cargo build --features="integra8/tokio-runtime"
cargo build --features="integra8/async-std-runtime"
popd

echo "Running Examples"
pushd "${EXAMPLES_ROOT}/target/debug"

./validate_examples
popd




