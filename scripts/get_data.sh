#!/usr/bin/env bash
# Downloads data files to ./data directory. You must ensure AOC_TOKEN env variable is set. This can
# be retrieved by retriving the value of the 'session' cookie when logged into the AOC website

set -xeuo pipefail

mkdir -p data
for i in {1..12}; do
    filename=$(printf "data/d%02d.txt" "$i")
    curl -H "Cookie: session=$AOC_TOKEN" "https://adventofcode.com/2025/day/$i/input" > "$filename"
done

