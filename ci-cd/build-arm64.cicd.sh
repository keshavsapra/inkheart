#!/bin/sh

if [ -z "$CI_REGISTRY_IMAGE" ] || [ -z "$CI_COMMIT_SHA" ]; then
    echo "Required environment variables CI_REGISTRY_IMAGE and CI_COMMIT_SHA are not set."
    echo "This script should not be run outside of a GitLab pipeline."
    exit 1
fi

echo "========================================="
echo "Building ARM64 Image"
echo "========================================="
echo "Commit SHA: $CI_COMMIT_SHA"
echo ""
echo "   NOTE: Building via QEMU emulation"
echo "   First build without cache: ~50-60 minutes"
echo "   With cache: ~25-30 minutes"
echo "========================================="

echo ""
echo "=== Installing QEMU for ARM64 Emulation ==="
echo ""

docker run --rm --privileged multiarch/qemu-user-static --reset -p yes

if [ $? -ne 0 ]; then
    echo "WARNING: QEMU installation returned non-zero exit code"
    echo "Continuing anyway - runner may already have QEMU installed"
fi

echo ""
echo "=== Building ARM64 Rust Layer ==="
echo ""

docker buildx build --progress=plain $PWD -f docker/dockerfile \
  --platform linux/arm64 \
  --target rust_builder \
  --cache-from type=registry,ref=$CI_REGISTRY_IMAGE/cargo-arm64:cache \
  --cache-to type=registry,ref=$CI_REGISTRY_IMAGE/cargo-arm64:cache,mode=max \
  --load \
  --tag rust-builder-arm64

if [ $? -ne 0 ]; then
    echo "ERROR: ARM64 rust_builder stage failed"
    exit 1
fi

echo ""
echo "=== Building ARM64 Final Image ==="
echo ""

docker buildx build $PWD -f docker/dockerfile \
  --platform linux/arm64 \
  --cache-from type=registry,ref=$CI_REGISTRY_IMAGE/cargo-arm64:cache \
  --cache-from type=registry,ref=$CI_REGISTRY_IMAGE/npm:cache \
  --tag $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-arm64 \
  --load

if [ $? -ne 0 ]; then
    echo "ERROR: ARM64 final build failed"
    exit 1
fi

echo ""
echo "OK: ARM64 build completed successfully"

echo ""
echo "=== Saving ARM64 Image ==="
echo ""

mkdir -p image

docker save $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-arm64 > image/nextchapter-arm64.tar
echo "OK: Saved ARM64 image to image/nextchapter-arm64.tar"

echo ""
echo "========================================="
echo "ARM64 Build Complete"
echo "========================================="
echo "OK: ARM64 image: $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-arm64"
echo "OK: Image tar: image/nextchapter-arm64.tar"
echo "========================================="