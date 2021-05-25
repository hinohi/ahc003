#!/bin/bash

set -eu

base=$(dirname "$0")

find "${base}/tools/in/" -name '*.txt' \
  | xargs -P 8 -I@ sh -c "$base/target/release/solver @ 2>&1 >/dev/null" \
  | awk '{print $0; s+=$3} END{print s}'
