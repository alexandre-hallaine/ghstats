#!/bin/bash

# Usage: ./process YEAR [MONTH] [DAY]
year=$(printf "%04d" "$1")
month=$(printf "%02d" "${2:-1}")
day=$(printf "%02d" "${3:-1}")

case $# in
  1) end_date="$year-12-31" ;;
  2) end_date=$(date -d "$year-$month-01 +1 month -1 day" +%F) ;;
  *) end_date="$year-$month-$day" ;;
esac

current="$year-$month-$day"
while [[ "$current" != $(date -I -d "$end_date + 1 day") ]]; do
  ./scripts/core.sh "$current" &
  (( $(jobs -r | wc -l) >= 8 )) && wait
  current=$(date -I -d "$current + 1 day")
done

wait
