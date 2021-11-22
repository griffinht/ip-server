#!/bin/bash

cd ../..
docker build . -f ci/docker/Dockerfile --tag ip-server-docker:latest
docker run ip-server-docker
