#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
  printf 'usage: %s <version>\n' "$0" >&2
  exit 1
fi

version="$1"

cargo set-version "$version"
cargo check # ensures Cargo.lock is update
git add Cargo.toml Cargo.lock
git commit -m "Bump version to v$version"
git push
tag="v$version"
git tag "$tag"
git push origin "$tag"
