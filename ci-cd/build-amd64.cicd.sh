#!/bin/sh

if [ -z "$CI_REGISTRY_IMAGE" ] || [ -z "$CI_COMMIT_SHA" ]; then
    echo "Required environment variables CI_REGISTRY_IMAGE and CI_COMMIT_SHA are not set."
    echo "This script should not be run outside of a GitLab pipeline."
    exit 1
fi

echo "========================================="
echo "Building AMD64 Image"
echo "========================================="
echo "Commit SHA: $CI_COMMIT_SHA"
echo "========================================="

echo ""
echo "=== Building AMD64 Rust Layer ==="
echo ""

docker buildx build --progress=plain $PWD -f docker/dockerfile \
  --platform linux/amd64 \
  --target rust_builder \
  --cache-from type=registry,ref=$CI_REGISTRY_IMAGE/cargo:cache \
  --cache-to type=registry,ref=$CI_REGISTRY_IMAGE/cargo:cache,mode=max \
  --load \
  --tag rust-builder

if [ $? -ne 0 ]; then
    echo "ERROR: AMD64 rust_builder stage failed"
    exit 1
fi

echo ""
echo "=== Building AMD64 Node Layer ==="
echo ""

docker buildx build $PWD -f docker/dockerfile \
  --platform linux/amd64 \
  --target node_builder \
  --cache-from type=registry,ref=$CI_REGISTRY_IMAGE/npm:cache \
  --cache-to type=registry,ref=$CI_REGISTRY_IMAGE/npm:cache,mode=max \
  --load \
  --tag node-builder

if [ $? -ne 0 ]; then
    echo "ERROR: AMD64 node_builder stage failed"
    exit 1
fi

echo ""
echo "=== Building AMD64 Final Image ==="
echo ""

docker buildx build $PWD -f docker/dockerfile \
  --platform linux/amd64 \
  --cache-from type=registry,ref=$CI_REGISTRY_IMAGE/cargo:cache \
  --cache-from type=registry,ref=$CI_REGISTRY_IMAGE/npm:cache \
  --tag $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA \
  --load

if [ $? -ne 0 ]; then
    echo "ERROR: AMD64 final build failed"
    exit 1
fi

echo ""
echo "OK: AMD64 build completed successfully"

echo ""
echo "=== Saving AMD64 Image ==="
echo ""

mkdir -p image

docker save $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA > image/inkheart.tar
echo "OK: Saved AMD64 image to image/inkheart.tar"

echo ""
echo "=== Exporting Build Artifacts ==="
echo ""

docker buildx build $PWD -f docker/dockerfile \
  --platform linux/amd64 \
  --target export \
  --cache-from type=registry,ref=$CI_REGISTRY_IMAGE/cargo:cache \
  --cache-from type=registry,ref=$CI_REGISTRY_IMAGE/npm:cache \
  --output type=local,dest=build

if [ $? -ne 0 ]; then
    echo "ERROR: Build artifact export failed"
    exit 1
fi

echo ""
echo "========================================="
echo "AMD64 Build Complete"
echo "========================================="
echo "OK: AMD64 image: $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA"
echo "OK: Image tar: image/inkheart.tar"
echo "OK: Build artifacts: build/"
echo "========================================="