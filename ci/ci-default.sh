#!/bin/bash

CARGO_TOKEN=$1
DOCKER_PASSWORD=$2

./ci/ci.sh "$CARGO_TOKEN" stzups "$DOCKER_PASSWORD" stzups/"$(./ci/cargo-metadata.sh name)":"$(./ci/cargo-metadata.sh version)"