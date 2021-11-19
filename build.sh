#!/usr/bin/env bash

cd py-rammer
cargo build --release
cp target/release/libpyrammer.dylib ../pyrammer.so
