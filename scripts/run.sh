#!/bin/bash
[ $# -ne 3 ] && echo "Usage: $0 year month day" && exit 1

./scripts/download.sh $1 $2 $3
./scripts/analyze.sh $1 $2 $3 
