#!/bin/bash

cd ../..
docker build . -f ci/docker/Dockerfile --tag ip-server-docker:latest
docker run  -e "DOCKER_REGISTRY_USER=$1" -e "DOCKER_REGISTRY_PASSWORD=$2" ip-server-docker
