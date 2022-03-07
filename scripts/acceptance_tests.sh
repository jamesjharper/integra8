
#! /usr/bin/env bash
set -e


Usage()
{
    echo ""
    echo "Required Parameters are missing"
    echo ""
    echo "Usage: $0 config async-runtime [clean]"
    echo " config = release, debug"
    echo " async-runtime = tokio, async-std"
    echo " clean"
    echo ""
}

# Arg 1 - Config
if [ "$1" == "debug" ]; then
    CONFIG="debug"
    BUILD_FLAGS=""

elif [ "$1" == "release" ]; then
    CONFIG="release"
    BUILD_FLAGS="--release"
else
    Usage && exit 1
fi

# Arg 2 - Runtime
if [ "$2" == "tokio" ]; then
    RUNTIME_NAME="tokio"
    BUILD_FEATURES="tokio-runtime integra8/tokio-runtime"
    CARGO_TARGET_DIR="./target/tokio/"
elif [ "$2" == "async-std" ]; then
    RUNTIME_NAME="async-std"
    BUILD_FEATURES="async-std-runtime integra8/async-std-runtime"
    CARGO_TARGET_DIR="./target/async-std/"
else
    Usage && exit 1
fi

# Arg 3 - Clean?
if [[ ! -z "$3" ]]; then
  CLEAN="true"
else
  CLEAN="false"
fi

EXAMPLES_ROOT="./examples"

if [ "$CLEAN" == "true" ]; then
    pushd "${EXAMPLES_ROOT}"
    echo " # Cleaning ${RUNTIME_NAME} ... "
    CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clean $RELEASE_FLAGS
    popd
fi

echo " # Building ${RUNTIME_NAME} ..."
pushd "${EXAMPLES_ROOT}"
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo build $RELEASE_FLAGS --features="${BUILD_FEATURES}"
popd

echo " # Running ${RUNTIME_NAME} ... "
pushd "${EXAMPLES_ROOT}"
pushd "${CARGO_TARGET_DIR}"
pushd "${CONFIG}"
./acceptance_tests
popd
popd
popd

