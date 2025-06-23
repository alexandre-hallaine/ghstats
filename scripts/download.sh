#!/bin/bash

[ $# -ne 3 ] && echo "Usage: $0 year month day" && exit 1

DATE=$(printf "%04d-%02d-%02d" $1 $2 $3)
DIR="data/$DATE"
mkdir -p "$DIR"

for hour in {0..23}; do
  FILE="$DIR/$DATE-$hour.json.gz"
  [ -f "$FILE" ] && continue
  
  TEMP=$(mktemp)
  curl -s -o "$TEMP" "https://data.gharchive.org/$DATE-$hour.json.gz" && mv "$TEMP" "$FILE"
done
