#!/bin/bash

[ $# -ne 1 ] && echo "Usage: $0 date" && exit 1

TMP=$(mktemp -d)
DIR="data/$1"
ARGS=()

mkdir -p data
[ -d "$DIR" ] && exit 0

for hour in {0..23}; do
  ARGS+=(-o "$TMP/$hour.json.gz" "https://data.gharchive.org/$1-$hour.json.gz")
done

curl --parallel --retry 99 --connect-timeout 10 "${ARGS[@]}"
gzip -d "$TMP"/*
mv "$TMP" "$DIR"
