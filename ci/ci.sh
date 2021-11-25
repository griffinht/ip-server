#!/bin/bash
set -e

CARGO_TOKEN=$1
USER=$2
NAME=$3
VERSION=$4
PASSWORD=$5

./ci/cargo/publish.sh "$CARGO_TOKEN"

./ci/docker/build-and-push.sh "$USER" "$NAME" "$VERSION" "$PASSWORD"

./ci/test/test-server-client.sh "$USER/$NAME:$VERSION"