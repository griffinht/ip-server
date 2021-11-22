FROM docker

COPY ci/docker/publish.sh .
ENTRYPOINT ["sh", "-c", "./publish.sh $DOCKER_REGISTRY_USER $DOCKER_REGISTRY_PASSWORD"]