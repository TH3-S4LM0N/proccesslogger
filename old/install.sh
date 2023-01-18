#!/usr/bin/env bash

cargo build --release
cp ./etc/processlogger.service /etc/systemd/user
cp ./target/release/processlogger /usr/bin/processlogger