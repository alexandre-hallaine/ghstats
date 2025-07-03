#!/bin/bash

# Usage: ./process YEAR [MONTH] [DAY]
start=$(printf "%04d-%02d-%02d" "$1" "${2:-1}" "${3:-1}")

case $# in
1) end=$(date -d "$start +1 year" +%F) ;;
2) end=$(date -d "$start +1 month" +%F) ;;
*) end=$(date -d "$start +1 day" +%F) ;;
esac

while [[ "$start" < "$end" ]]; do
  echo "$start"
  start=$(date -d "$start +1 day" +%F)
done | parallel --eta ./scripts/core.sh {}
