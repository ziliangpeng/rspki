#! /bin/bash

# test with 2 ^ [7, 7.5, 8, 8.5, 9, 9.5, 10, ...]
# 128, 181, 256, 362, 512, ...
for bits in 128 181 256 362 512; do
    echo "Benchmarking prime generation for ${bits} bits..."
    total_time=0
    for i in {1..100}; do
        start=$(date +%s.%N)
        bash ./run.sh "$bits" > /dev/null 2>/dev/null
        end=$(date +%s.%N)
        elapsed=$(echo "$end - $start" | bc)
        total_time=$(echo "$total_time + $elapsed" | bc)
    done
    avg_time=$(echo "$total_time / 100" | bc -l)
    echo "Average run time: ${avg_time} seconds"
done
