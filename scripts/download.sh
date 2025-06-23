#!/bin/bash

[ $# -ne 3 ] && echo "Usage: $0 year month day" && exit 1

DATE=$(printf "%04d-%02d-%02d" "$1" "$2" "$3")
DIR="data/$DATE"
mkdir -p "$DIR"

args=()
for hour in {0..23}; do
  f="$DIR/$hour.json.gz"
  [ -f "$f" ] && continue
  args+=(-o "$f" "https://data.gharchive.org/$DATE-$hour.json.gz")
done

[ ${#args[@]} -gt 0 ] && curl --parallel --fail -sS "${args[@]}"
