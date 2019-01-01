#!/bin/bash

cargo build --release || exit 1
for f in $(grep name Cargo.toml | sed '/dice/d' | awk '{print $3}' | sed 's/"//g')
do
    echo -e "\033[32m"Running $f"\033[0m"
    target/release/$(basename $f '.rs') >/dev/null
done
