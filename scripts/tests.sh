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
    echo " features = 1_by_1, all_at_once"
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



# Arg 3 - Target features
if [ "$3" == "1_by_1" ]; then
    # Test each feature 1 by 1,
    # which give the additonal assurance 
    # the feature flags are working correctly
    TARGET_FEATURES=(
        "components"
        "async_runtime"
        "decorations"
        "results"
        "scheduling"
        "runner"
        "formatters"
    )
elif [ "$3" == "all_at_once" ]; then
    # Test all at once, which is faster,
    # but wont check feature flags are working 
    # correctly
    TARGET_FEATURES=(
        "core"
    )
else
    Usage && exit 1
fi

# Arg 3 - Clean?
if [[ ! -z "$4" ]]; then
    echo " # Cleaning ${RUNTIME_NAME} (${CONFIG}) ... "
    CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clean $BUILD_FLAGS
fi

for FEATURE in "${TARGET_FEATURES[@]}"; do
    echo " # Testing feature \"${FEATURE}\" for ${RUNTIME_NAME} (${CONFIG}) ..."
    CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo test $BUILD_FLAGS --no-default-features --features="${BUILD_FEATURES} ${FEATURE}"
done


