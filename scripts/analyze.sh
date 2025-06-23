#!/bin/bash

[ $# -ne 1 ] && echo "Usage: $0 date" && exit 1

DIR="data/$1"
[ ! -d "$DIR" ] && echo "No data found. Run download.sh first." && exit 2

mkdir -p output
cargo build --release &>/dev/null
./target/release/ghlang "$DIR" >"output/$1.json"
