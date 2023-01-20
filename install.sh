#!/usr/bin/env bash
set -eu

echo "Install Path: $1"
cargo build --release
cp ./target/release/{pl-daemon, pl-web} $1
mkdir ~/.config/processlogger/
cp config.toml ~/.config/processlogger/