#!/bin/bash
set -e

CARGO_TOKEN=$1

docker build . -f ./ci/cargo/publish/Dockerfile --tag ip-server-cargo-publish:latest
docker run --rm ip-server-cargo-publish:latest "$CARGO_TOKEN"

docker build . -f ./ci/cargo/install/Dockerfile --tag ip-server-cargo-install:latest --build-arg "VERSION=$(./ci/cargo-metadata.sh version)"
./ci/test/test-server-client.sh ip-server-cargo-install:latest