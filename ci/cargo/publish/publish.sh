#!/bin/bash
set -e

CARGO_TOKEN=$1

if [ -n "$CARGO_TOKEN" ]; then
  cargo login "$CARGO_TOKEN"
fi
cargo publish