#!/bin/bash

DAY=$(env TZ=America/Toronto date +%-d)
DAY_PADDED=$(printf "%02d" $DAY)

firefox https://adventofcode.com/2021/day/${DAY}

cargo run -p update_workspace -- ./Cargo.toml day${DAY_PADDED}
cargo new day${DAY_PADDED}
cargo run -p template_manifest -- ./day${DAY_PADDED}/Cargo.toml

cp template.rs day${DAY_PADDED}/src/main.rs

curl "https://adventofcode.com/2021/day/${DAY}/input" -H "Cookie: session=${ADVENT_SESSION}" > "day${DAY_PADDED}/input"

code -n . day${DAY_PADDED}/src/main.rs day${DAY_PADDED}/input

cd day${DAY_PADDED} && cargo build && cargo build --release
