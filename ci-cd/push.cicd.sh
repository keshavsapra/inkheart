#!/bin/sh

if [ -z "$CI_REGISTRY_IMAGE" ] || [ -z "$CI_COMMIT_SHA" ]; then
    echo "Required environment variables not set"
    exit 1
fi

echo "========================================="
echo "Publishing Images to Registries"
echo "========================================="

PACKAGE_VERSION=$(grep ^version= VERSION | cut -d'=' -f2 | tr -d '[:space:]')
echo "Version: $PACKAGE_VERSION"

echo ""
echo "=== Loading AMD64 Image ==="

# Load AMD64 image (NO suffix in tar)
docker load -i image/nextchapter.tar
# Tag it WITH suffix for publishing
docker tag $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-amd64
echo "OK: Loaded and tagged AMD64 image"

echo ""
echo "=== Checking for ARM64 Image ==="

HAS_ARM64=false
if [ -f image/nextchapter-arm64.tar ] && [ -s image/nextchapter-arm64.tar ]; then
    # ARM64 already has suffix in tar
    docker load -i image/nextchapter-arm64.tar
    HAS_ARM64=true
    echo "OK: ARM64 image found and loaded (multi-arch build)"
else
    echo "SKIP: ARM64 image not found (AMD64-only build)"
fi

echo ""
echo "=== Pushing to GitLab Registry ==="

docker push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-amd64
echo "OK: Pushed AMD64 to GitLab Registry"

if [ "$HAS_ARM64" = "true" ]; then
    docker push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-arm64
    echo "OK: Pushed ARM64 to GitLab Registry"
    
    echo ""
    echo "=== Creating Multi-Arch Manifest (GitLab Registry) ==="
    
    docker buildx imagetools create -t $CI_REGISTRY_IMAGE:${PACKAGE_VERSION} \
      $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-amd64 \
      $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-arm64
    
    docker buildx imagetools create -t $CI_REGISTRY_IMAGE:latest \
      $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-amd64 \
      $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-arm64
    
    echo "OK: Created multi-arch manifest (AMD64 + ARM64)"
else
    echo ""
    echo "=== Publishing AMD64-Only Images (GitLab Registry) ==="
    
    docker tag $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-amd64 $CI_REGISTRY_IMAGE:${PACKAGE_VERSION}
    docker push $CI_REGISTRY_IMAGE:${PACKAGE_VERSION}
    
    docker tag $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-amd64 $CI_REGISTRY_IMAGE:latest
    docker push $CI_REGISTRY_IMAGE:latest
    
    echo "OK: Published AMD64-only images"
fi

echo ""
echo "=== Pushing to Docker Hub ==="

docker login -u $DOCKERHUB_USER -p $DOCKERHUB_PASSWORD $DOCKERHUB_REGISTRY

# Tag and push AMD64
docker tag $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-amd64 $DOCKERHUB_USER/nextchapter:${PACKAGE_VERSION}
docker tag $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-amd64 $DOCKERHUB_USER/nextchapter:latest
docker push $DOCKERHUB_USER/nextchapter:${PACKAGE_VERSION}
docker push $DOCKERHUB_USER/nextchapter:latest
echo "OK: Pushed AMD64 to Docker Hub"

if [ "$HAS_ARM64" = "true" ]; then
    # Tag and push ARM64
    docker tag $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-arm64 $DOCKERHUB_USER/nextchapter:${PACKAGE_VERSION}-arm64
    docker push $DOCKERHUB_USER/nextchapter:${PACKAGE_VERSION}-arm64
    echo "OK: Pushed ARM64 to Docker Hub"
    
    echo ""
    echo "=== Creating Multi-Arch Manifest (Docker Hub) ==="
    
    docker buildx imagetools create -t $DOCKERHUB_USER/nextchapter:${PACKAGE_VERSION} \
      $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-amd64 \
      $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-arm64
    
    docker buildx imagetools create -t $DOCKERHUB_USER/nextchapter:latest \
      $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-amd64 \
      $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA-arm64
    
    echo "OK: Created multi-arch manifest on Docker Hub"
    
    echo ""
    echo "=== Verifying Multi-Arch Manifest ==="
    docker buildx imagetools inspect $DOCKERHUB_USER/nextchapter:${PACKAGE_VERSION}
else
    echo "OK: Published AMD64-only images to Docker Hub"
fi

echo ""
echo "========================================="
echo "Publish Complete"
echo "========================================="
if [ "$HAS_ARM64" = "true" ]; then
    echo "OK: Multi-arch images published (AMD64 + ARM64)"
    echo "  GitLab: $CI_REGISTRY_IMAGE:${PACKAGE_VERSION}"
    echo "  Docker Hub: $DOCKERHUB_USER/nextchapter:${PACKAGE_VERSION}"
else
    echo "OK: AMD64-only images published"
    echo "  GitLab: $CI_REGISTRY_IMAGE:${PACKAGE_VERSION}"
    echo "  Docker Hub: $DOCKERHUB_USER/nextchapter:${PACKAGE_VERSION}"
fi
echo "========================================="