#!/bin/bash
set -e

docker build . -f ./ci/deb/Dockerfile --tag ip-server-deb:latest
./ci/test/test-server-client.sh ip-server-deb:latest