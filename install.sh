#!/usr/bin/env bash
cargo build --release
cp ./target/release/proccesslogger /usr/bin/proccesslogger