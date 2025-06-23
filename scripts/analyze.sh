#!/bin/bash

[ $# -ne 3 ] && echo "Usage: $0 year month day" && exit 1

DATE=$(printf "%04d-%02d-%02d" $1 $2 $3)
DIR="data/$DATE"
[ ! -d "$DIR" ] && echo "No data found. Run download.sh first." && exit 2

mkdir -p output
cargo build --release
./target/release/ghlang "$DIR" >"output/$DATE.json"
