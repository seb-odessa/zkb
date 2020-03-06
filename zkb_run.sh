#!/bin/bash

export ZKB_INTERFACE=127.0.0.1:8088
export DATABASE_URL=navigator.db

wget -qO- "http://${ZKB_INTERFACE}/navigator/cmd/quit"
cargo build
RUST_LOG=info ./target/debug/zkb &

