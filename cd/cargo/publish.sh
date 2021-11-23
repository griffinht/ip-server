#!/bin/bash

TOKEN=$1
echo "hello"
echo "$1" "$2"
cargo login "$TOKEN"
cargo publish