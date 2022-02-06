
#! /usr/bin/env bash
set -e

# Root of workspace-like directories.
CORE_ROOT="./core"
CONTRIB_ROOT="./contrib/"

ALL_CRATE_ROOTS=(
    "${CORE_ROOT}/async_runtime"
    "${CORE_ROOT}/components"
    "${CORE_ROOT}/decorations_impl"
    "${CORE_ROOT}/decorations"
    "${CORE_ROOT}/results"
    "${CORE_ROOT}/scheduling"
    "${CORE_ROOT}/runner"
    "${CORE_ROOT}/formatters"
    "${CONTRIB_ROOT}/formatters/tree_formatter"
    "${CORE_ROOT}/integra8_impl"
    "${CORE_ROOT}/integra8"
)

#
# Publishes all Integra8 crates to crates.io.
#
if ! [ -z "$(git status --porcelain)" ]; then
  echo "Uncommitted changes, please commit and run again."
  #exit 1
fi

echo "Cleaning"
cargo clean

#./scripts/build.sh
#./scripts/tests.sh
#./scripts/run_examples.sh

# Publish all the things.
for dir in "${ALL_CRATE_ROOTS[@]}"; do
  pushd "${dir}"
  echo "Publishing '${dir}'..."
  cargo publish --no-verify --allow-dirty ${@:1} # --dry-run 
  echo "Waiting before trying next upload"
  sleep 120
  popd
done