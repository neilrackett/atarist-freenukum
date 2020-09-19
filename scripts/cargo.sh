#!/bin/sh

export OUTPUT="$2"
export CARGO_TARGET_DIR="$3"/target
export CARGO_HOME="$CARGO_TARGET_DIR"/cargo-home
export BUILD_PROFILE="$4"

if [ "$BUILD_PROFILE" = "debug" ]
then
    echo "DEBUG MODE"
    cargo build --manifest-path "$1"/Cargo.toml -p freenukum-rs && cp "$CARGO_TARGET_DIR"/debug/libfreenukum_rs.so "$OUTPUT"
else
    echo "RELEASE MODE"
    cargo build --manifest-path "$1"/Cargo.toml --release -p freenukum-rs && cp "$CARGO_TARGET_DIR"/release/libfreenukum_rs.so "$OUTPUT"
fi
