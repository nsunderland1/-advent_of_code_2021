#!/bin/bash

DAY=$(env TZ=America/Toronto date +%-d)
DAY_PADDED=$(printf "%02d" $DAY)

firefox https://adventofcode.com/2021/day/${DAY}

curl "https://adventofcode.com/2021/day/${DAY}/input" -H "Cookie: session=${ADVENT_SESSION}" > "input/${DAY}.input"

code -n . src/day${DAY_PADDED}.rs input/${DAY}.input

cargo build && cargo build --release
