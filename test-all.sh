#!/bin/bash

set -eu

base=$(dirname "$0")/target/release

find tools/in/ -name '*.txt' \
  |  xargs -P 8 -I@ sh -c "$base/tester @ $base/solver 2>&1 >/dev/null" \
  | awk '{print $0; s+=$3} END{print s}'