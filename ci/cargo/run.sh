#!/bin/bash

cd ../..
docker build . -f ci/cargo/Dockerfile --tag ip-server-cargo:latest
docker run ip-server-cargo
