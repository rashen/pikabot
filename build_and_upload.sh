#!/usr/bin/bash

set -e # Abort at fail

TARGET=arm-unknown-linux-gnueabihf
TOOLCHAIN=cross-pi-gcc-8.3.0-1
TARGET_DIR=~/pikabot/.
HOST=rpi

PATH=$HOME/toolchains/${TOOLCHAIN}/bin:$PATH \
    cargo build --release --target $TARGET

echo "Transfering binary and dependencies"
scp -r ./target/$TARGET/release/pikabot ${HOST}:${TARGET_DIR}
# scp .token ${HOST}:${TARGET_DIR}
# scp .public_key ${HOST}:${TARGET_DIR}
# scp .app_id ${HOST}:${TARGET_DIR}
