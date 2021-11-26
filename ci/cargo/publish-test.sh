#!/bin/bash
set -e

CARGO_TOKEN=$1

docker build . -f ./ci/cargo/Dockerfile --tag ip-server-cargo:latest --build-arg "CARGO_TOKEN=$CARGO_TOKEN"
./ci/test/test-server-client.sh ip-server-cargo:latest