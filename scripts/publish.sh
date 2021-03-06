
#! /usr/bin/env bash
set -e

# Root of workspace-like directories.
CORE_ROOT="./core"
CONTRIB_ROOT="./contrib/"

ALL_CRATE_ROOTS=(
    "${CONTRIB_ROOT}/formatters/serde_formatter"
    "${CONTRIB_ROOT}/formatters/tree_formatter"
    "${CORE_ROOT}/decorations_impl"  
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

echo "Updating cargo.lock"
cargo update

echo "Cleaning"
cargo clean

./scripts/build.sh release tokio
./scripts/build.sh release async-std

./scripts/tests.sh release tokio 1_by_1
./scripts/tests.sh release async-std 1_by_1

./scripts/acceptance_tests.sh release tokio
./scripts/acceptance_tests.sh release async-std

# Publish all the things.
for dir in "${ALL_CRATE_ROOTS[@]}"; do
  pushd "${dir}"
  echo "Publishing '${dir}'..."
  cargo publish --no-verify --allow-dirty ${@:1} # --dry-run 
  echo "Waiting before trying next upload"
  sleep 120
  popd
done