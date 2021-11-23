#!/bin/bash

cd ../..
docker build . -f ci/cargo/Dockerfile --tag cargo-publish:latest
docker run -e "CARGO_TOKEN=$1" cargo-publish:latest
