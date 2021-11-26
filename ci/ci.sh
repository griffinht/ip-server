#!/bin/bash
set -e

CARGO_TOKEN=$1
IMAGE=$2
DOCKER_REGISTRY_USER=$3
DOCKER_REGISTRY_PASSWORD=$4


./ci/cargo/publish.sh "$CARGO_TOKEN"

./ci/docker/build-and-push.sh "$IMAGE" "$DOCKER_REGISTRY_USER" "$DOCKER_REGISTRY_PASSWORD"

./ci/test/test-server-client.sh "$IMAGE"