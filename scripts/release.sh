#!/bin/bash
set -e

# Usage: ./scripts/release.sh <version>
VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version> (e.g. 0.2.0)"
    exit 1
fi

# Ensure version starts with 'v' for git tags
TAG="v$VERSION"

echo "Releasing $TAG..."

# 1. Bump version in root Cargo.toml (all packages inherit via workspace.package.version)
sed -i 's/^version = ".*"/version = "'"$VERSION"'"/' Cargo.toml

# 2. Update Cargo.lock
cargo check -q

# 4. Git operations
git add .
git commit -m "chore(release): prepare for $TAG"
git tag -a "$TAG" -m "Release $TAG"

echo "--------------------------------------------------"
echo "Release $TAG prepared locally."
echo "Version bumped in Cargo.toml files."
echo "Changelog generated."
echo "Git tag created."
echo "--------------------------------------------------"

read -p "Do you want to push the changes and tags? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin main && git push origin "$TAG"
    echo "Successfully pushed!"
else
    echo "Push cancelled. You can push manually with 'git push origin main && git push origin $TAG'."
fi
