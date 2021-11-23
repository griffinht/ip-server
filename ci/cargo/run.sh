#!/bin/bash
set -e

TOKEN=$1

docker build . -f ./ci/cargo/Dockerfile --tag cargo-docker:latest
docker run -e "CARGO_TOKEN=$TOKEN" cargo-docker:latest
