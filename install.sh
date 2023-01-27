#!/usr/bin/env bash
set -eu

echo "Install Path: $1"
cargo build --release
cp ./target/release/{pl-daemon, pl-web} $1
mkdir ~/.config/processlogger/
cp config.ron ignorefile.ron ~/.config/processlogger/
mkdir -p ~/.config/systemd/user
cat <<EOF
[Unit]
Description=processlogger

[Service]
ExecStart=$1/pl-daemon
Restart=always
RestartSec=12

[Install]
WantedBy=default.target
EOF > ~/.config/systemd/user/processlogger.service