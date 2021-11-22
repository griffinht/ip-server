#!/bin/bash

USER=$1
PASSWORD=$2

docker build . --tag ip-server:latest
echo "$PASSWORD" | docker login -u "$USER" --password-stdin
docker push "$USER"/ip-server:latest