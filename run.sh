#!/bin/bash

DAY=$(env TZ=America/Toronto date +%-d)
DAY_PADDED=$(printf "%02d" $DAY)

cargo run -p day${DAY_PADDED}
