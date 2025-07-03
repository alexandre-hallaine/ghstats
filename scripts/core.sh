#!/bin/bash

# Usage: ./core YYYY-MM-DD
[[ $1 =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || exit 1

mkdir -p stats temp_$1
cd temp_$1

for h in {0..23}; do
  curl -sO https://data.gharchive.org/$1-$h.json.gz
  gunzip $1-$h.json.gz
done

cargo run -r -- . > ../stats/$1.json
cd .. && rm -rf temp_$1
