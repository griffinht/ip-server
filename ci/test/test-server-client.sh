#!/bin/bash
set -e

# starts server and connects client to server
# see exit code for result
USER=$1
NAME=$2
VERSION=$3

SERVER_ID=$(docker run --rm -p 8000:8000 --detach "$USER/$NAME:$VERSION")
docker run --rm --network=host "$USER/$NAME:$VERSION" --client 127.0.0.1:8000
CLIENT_EXIT_CODE=$?
docker rm "$SERVER_ID" --force > /dev/null
exit "$CLIENT_EXIT_CODE"