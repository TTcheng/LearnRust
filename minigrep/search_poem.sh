#!/usr/bin/env bash

cargo build
./target/debug/minigrep nobody poem.txt

# equals to `cargo run nobody poem.txt`