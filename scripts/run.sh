#!/bin/bash

[ $# -ne 2 ] && echo "Usage: $0 year month" && exit 1

for day in $(seq 1 31); do
  DATE=$(printf "%04d-%02d-%02d" "$1" "$2" "$day")
  ! date -d "$DATE" &>/dev/null && continue

  echo "[$DATE] Downloading..."
  ./scripts/download.sh "$DATE" >/dev/null 2>&1

  echo "[$DATE] Analyzing..."
  ./scripts/analyze.sh "$DATE" >/dev/null 2>&1

  rm -rf "data/$DATE"
done
