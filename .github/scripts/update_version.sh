#!/bin/sh
set -eu

if [ $# -lt 1 ]; then
    echo "Usage: $0 <version>" >&2
    exit 1
fi
VERSION=$1

sed -i.bak -e "s/^version *=.*/version = \"${VERSION}\"/" Cargo.toml
rm -f Cargo.toml.bak

grep "^version" Cargo.toml
