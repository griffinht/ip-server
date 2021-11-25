#!/bin/bash
set -e

TOKEN=$1

if [ -n "$TOKEN" ]; then
  cargo login "$TOKEN"
fi
cargo publish --allow-dirty