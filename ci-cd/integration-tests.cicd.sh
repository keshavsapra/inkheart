#!/bin/sh

# Check if required environment variables are set
if [ -z "$CI_REGISTRY_IMAGE" ] || [ -z "$CI_COMMIT_SHA" ]; then
    echo "Required environment variables CI_REGISTRY_IMAGE and CI_COMMIT_SHA are not set. This script should not be run outside of a gitlab pipeline. Exiting..."
    exit 1
fi

# put correct image name in compose config
sed -i "s@.*image:.*@    image: $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA@" docker-compose.yml

# create paths
mkdir -pv /path/to/{media,covers,config,thumbnails}

apk update && apk add curl grep

docker compose up -d

sleep 10

HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" http://docker:8080) && [[ $HTTP_CODE -eq 200 ]] || exit 1
curl -sS http://docker:8080 | grep --silent '<title>NextChapter</title>' || exit 1

# add more integration tests
# ...