#! /bin/bash

# test with 2 ^ [7, 7.5, 8, 8.5, 9, 9.5, 10, ...]
# 128, 181, 256, 362, 512, ...
for bits in 128 181 256 362 512 724; do
    echo "Benchmarking prime generation for ${bits} bits..."
    start=$(date +%s.%N)
    bash ./run.sh "$bits" 100 > /dev/null 2>/dev/null
    end=$(date +%s.%N)
    elapsed=$(echo "$end - $start" | bc)
    avg_time=$(echo "$elapsed / 100" | bc -l)
    echo "Average run time: ${avg_time} seconds"
done
