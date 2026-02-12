#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  printf 'usage: %s <version>\n' "$0" >&2
  exit 1
fi

version="$1"
tag="v$version"

cargo set-version "$version"
cargo check # ensures Cargo.lock is updated
git add Cargo.toml Cargo.lock
git commit -m "Bump version to $tag"
git push
git tag "$tag"
git push origin "$tag"
