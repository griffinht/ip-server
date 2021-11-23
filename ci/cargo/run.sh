#!/bin/bash

cd ../..
docker build . -f ci/cargo/Dockerfile --tag ip-server-cargo:latest
docker run -e "CARGO_TOKEN=$1" ip-server-cargo
