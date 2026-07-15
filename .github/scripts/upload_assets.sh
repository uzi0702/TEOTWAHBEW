#!/bin/sh
set -eu

if [ $# -lt 1 ]; then
    echo "Usage: $0 <version>" >&2
    exit 1
fi
TAG=v$1
REPO=${GITHUB_REPOSITORY:-uzi0702/TEOTWAHBEW}
API=https://api.github.com/repos/${REPO}

# Draft releases cannot be fetched via /releases/tags/<tag>, so search the list.
upload_url=$(curl -s -H "Authorization: Bearer ${GITHUB_TOKEN}" "${API}/releases?per_page=30" \
    | jq -r --arg tag "${TAG}" '[.[] | select(.tag_name == $tag)][0].upload_url // empty' \
    | sed 's/{.*//')

if [ -z "${upload_url}" ]; then
    echo "Error: release for tag ${TAG} not found" >&2
    exit 1
fi

for file in dist/*.tar.gz; do
    name=$(basename "${file}")
    echo "Uploading ${name}..."
    curl -sf -X POST \
        -H "Authorization: Bearer ${GITHUB_TOKEN}" \
        -H "Content-Type: application/gzip" \
        --data-binary @"${file}" \
        "${upload_url}?name=${name}" > /dev/null
done
echo "Done."
