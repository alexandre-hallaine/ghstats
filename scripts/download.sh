#!/bin/bash

[ $# -ne 1 ] && echo "Usage: $0 date" && exit 1

DIR="data/$1"
mkdir -p "$DIR"

args=()
for hour in {0..23}; do
  f="$DIR/$hour.json.gz"
  [ -f "$f" ] && continue
  args+=(-o "$f" "https://data.gharchive.org/$1-$hour.json.gz")
done

[ ${#args[@]} -gt 0 ] && curl --parallel \
  --fail \
  --retry 99 \
  --retry-delay 5 \
  --connect-timeout 10 \
  --speed-limit 1024 \
  --speed-time 30 \
  -sS "${args[@]}"
