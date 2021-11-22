#!/bin/bash

REGISTRY_PASSWORD=todo
REGISTRY_USER=stzups

docker build . --tag ip-server:latest
echo "$REGISTRY_PASSWORD" | docker login -u "$REGISTRY_USER" --password-stdin
docker push stzups/ip-server:latest