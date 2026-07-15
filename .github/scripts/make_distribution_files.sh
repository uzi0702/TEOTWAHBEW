#!/bin/sh
set -eu

if [ $# -lt 1 ]; then
    echo "Usage: $0 <version>" >&2
    exit 1
fi
VERSION=$1
NAME=teot-${VERSION}
DIST=dist

cargo build --release

rm -rf "${DIST}/${NAME}"
mkdir -p "${DIST}/${NAME}"
cp target/release/teot README.md LICENSE "${DIST}/${NAME}/"
tar -czf "${DIST}/${NAME}_linux_amd64.tar.gz" -C "${DIST}" "${NAME}"
rm -rf "${DIST}/${NAME}"

ls -l "${DIST}"
