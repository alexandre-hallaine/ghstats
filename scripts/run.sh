#!/bin/bash

[ $# -ne 2 ] && echo "Usage: $0 year month" && exit 1

for day in $(seq 1 31); do
  DATE=$(printf "%04d-%02d-%02d" "$1" "$2" "$day")

  if ! date -d "$DATE" &>/dev/null; then
    continue
  fi

  echo "Processing $DATE..."
  ./scripts/download.sh "$DATE"
  ./scripts/analyze.sh "$DATE"
  rm -rf "data/$DATE"
done
