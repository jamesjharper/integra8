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
    BUILD_FEATURES="tokio-runtime"
    CARGO_TARGET_DIR="./target/tokio/"
elif [ "$2" == "async-std" ]; then
    RUNTIME_NAME="async-std"
    BUILD_FEATURES="async-std-runtime"
    CARGO_TARGET_DIR="./target/async-std/"
else
    Usage && exit 1
fi

echo " # Building ${RUNTIME_NAME} (${CONFIG}) ..."
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo build $BUILD_FLAGS --features="${BUILD_FEATURES}"


