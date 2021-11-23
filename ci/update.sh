#!/bin/bash
set -e

VERSION=$1

if [[ $(git status --porcelain) ]]; then
  echo "git status shows changes, please commit/discard these changes to continue"
  exit 1
fi

OLD_VERSION=$(./ci/cargo-metadata.sh version)

if [[ "$VERSION" == "$OLD_VERSION" ]]; then
  echo "Cargo.toml is already version $VERSION"
  exit 1
fi

sed -i "s/$OLD_VERSION/$VERSION/g" Cargo.toml

git add Cargo.toml
git commit -m "version $OLD_VERSION -> $VERSION"
git push origin
git tag "$VERSION"
git push origin --tag