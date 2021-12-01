#!/bin/bash

DAY=$(env TZ=America/Toronto date +%-d)
DAY_PADDED=$(printf "%02d" $DAY)

firefox https://adventofcode.com/2020/day/${DAY}

cargo new day${DAY_PADDED}
cargo run -p update_workspace -- ./Cargo.toml day${DAY_PADDED}
cp template.rs day${DAY_PADDED}/src/main.rs

code -n . day${DAY_PADDED}/src/main.rs

curl "https://adventofcode.com/2021/day/${DAY}/input" -H "Cookie: session=${ADVENT_SESSION}" > "day${DAY_PADDED}/input"

cd day${DAY_PADDED} && cargo build && cargo build --release
