#!/bin/bash

[ $# -ne 2 ] && echo "Usage: $0 year month" && exit 1

for day in $(seq 1 31); do
  date_str="$1-$2-$day"

  if ! date -d "$date_str" &>/dev/null; then
    continue
  fi

  echo "Processing $date_str..."
  ./scripts/download.sh "$1" "$2" "$day"
  ./scripts/analyze.sh "$1" "$2" "$day"
done
