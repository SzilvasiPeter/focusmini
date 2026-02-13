#!/usr/bin/env bash
set -euo pipefail

version=${1:?usage: $0 <version>}
tag="v$version"

cargo set-version "$version"
git add Cargo.toml Cargo.lock
git commit -m "Bump version to $tag"
git tag "$tag"
git push origin HEAD "$tag"
