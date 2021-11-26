#!/bin/bash
set -e

IMAGE=$1
DOCKER_REGISTRY_USER=$2
DOCKER_REGISTRY_PASSWORD=$3

docker build -f ./ci/docker/Dockerfile --tag "$IMAGE" .
if [ -n "$DOCKER_REGISTRY_PASSWORD" ]; then
  echo "$DOCKER_REGISTRY_PASSWORD" | docker login -u "$DOCKER_REGISTRY_USER" --password-stdin
fi

docker push "$IMAGE"