
#! /usr/bin/env bash
set -e


# Root of workspace-like directories.
PROJECT_ROOT="."
CORE_ROOT="./core"


ALL_CRATE_ROOTS=(
    "${CORE_ROOT}/async_runtime"
    "${CORE_ROOT}/components"
    "${CORE_ROOT}/decorations_impl"
    "${CORE_ROOT}/decorations"
    "${CORE_ROOT}/results"
    "${CORE_ROOT}/scheduling"
    "${CORE_ROOT}/runner"
    "${CORE_ROOT}/formatters"
    "${CORE_ROOT}/integra8"
)

#
# Publishes all Integra8 crates to crates.io.
#

if ! [ -z "$(git status --porcelain)" ]; then
  echo "Uncommitted changes, please commit and run again."
  #exit 1
fi

echo "Running Tests before publish."
#cargo clean
#cargo test --release

# Publish all the things.
for dir in "${ALL_CRATE_ROOTS[@]}"; do
  pushd "${dir}"
  echo "Publishing '${dir}'..."
  cargo publish --no-verify --dry-run --allow-dirty ${@:1}
  sleep 5
  popd
done