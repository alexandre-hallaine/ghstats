#!/bin/bash

[ $# -ne 1 ] && echo "Usage: $0 date" && exit 1

mkdir -p data

TMP=$(mktemp -d ./data/tmp.XXXXXX)
DIR="data/$1"
ARGS=()

[ -d "$DIR" ] && exit 0

for hour in {0..23}; do
  ARGS+=(-o "$TMP/$hour.json.gz" "https://data.gharchive.org/$1-$hour.json.gz")
done

curl --parallel --retry 99 --connect-timeout 10 "${ARGS[@]}"
gzip -d "$TMP"/*
mv "$TMP" "$DIR"
