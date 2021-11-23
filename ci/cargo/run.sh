#!/bin/bash
set -e

CARGO_TOKEN=$1

docker build . -f ./ci/cargo/Dockerfile --tag cargo-docker:latest
docker run -e "CARGO_TOKEN=$CARGO_TOKEN" cargo-docker:latest
