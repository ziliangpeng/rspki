#! /bin/bash

N_BITS=${1:-128}

FILE_LOCATION=$(dirname $0)

cargo run --quiet --bin primegen -- --n_bits=$N_BITS | tail -n 1 | python $FILE_LOCATION/../verify/is_prime.py
