#!/usr/bin/env bash

cargo build --release
cp ./etc/proccesslogger.service /etc/systemd/user
cp ./target/release/proccesslogger /usr/bin/proccesslogger