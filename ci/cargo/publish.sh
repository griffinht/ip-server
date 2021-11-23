#!/bin/bash
set -e

TOKEN=$1

cargo login "$TOKEN"
cargo publish