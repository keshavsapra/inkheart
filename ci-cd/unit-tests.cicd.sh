#!/bin/sh

# Check if required environment variables are set
if [ -z "$CI_REGISTRY_IMAGE" ]; then
    echo "Required environment variable CI_REGISTRY_IMAGE is not set. This script should not be run outside of a gitlab pipeline. Exiting..."
    exit 1
fi

# Build test image with registry cache
docker buildx build $PWD -f docker/dockerfile \
    --target test \
    --cache-from type=registry,ref=$CI_REGISTRY_IMAGE/cargo-test:cache \
    --cache-to type=registry,ref=$CI_REGISTRY_IMAGE/cargo-test:cache,mode=max \
    --tag $CI_REGISTRY_IMAGE/cargo-test:latest \
    --load

# Run tests
docker run $CI_REGISTRY_IMAGE/cargo-test:latest