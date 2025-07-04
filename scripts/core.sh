#!/bin/bash

# Usage: ./core YYYY-MM-DD
[[ $1 =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || exit 1

mkdir -p stats temp_$1
cd temp_$1

curl --parallel --fail $(printf -- "-O https://data.gharchive.org/$1-%d.json.gz " {0..23})
parallel -j2 'gunzip {} || rm {}' ::: *

cargo run -r -- . >../stats/$1.json
cd .. && rm -rf temp_$1
