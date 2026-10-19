#!/usr/export/env sh

buildType=$1

# Check if a valid argument has been provided
if [ "$buildType" != "docker" ] && [ "$buildType" != "export" ] && [ "$buildType" != "test" ]; then
    echo "Invalid argument. Please provide 'docker', 'export', or 'test'."
    exit 1
fi

export DOCKER_BUILDKIT=1

version=$(grep ^version= VERSION | cut -d'=' -f2 | tr -d '[:space:]')

if [ "$buildType" = "docker" ]; then
    docker build . -f docker/dockerfile -t "inkheart:${version}" -m 2g
elif [ "$buildType" = "export" ]; then
    if [ ! -d "./build" ]; then
        mkdir "./build"
    fi
    docker build . -f docker/dockerfile --target export -o build -m 2g
elif [ "$buildType" = "test" ]; then
    docker build . -f docker/dockerfile -t inkheart-test --target test -m 2g
    docker run inkheart-test
fi