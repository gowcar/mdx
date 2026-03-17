#!/usr/bin/env bash
set -euo pipefail

# Get current version from Cargo.toml
CURRENT=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

if [ -n "${1:-}" ]; then
  # Specific version provided
  VERSION="${1#v}"
else
  # Auto-increment patch version: 0.1.3 -> 0.1.4
  IFS='.' read -r major minor patch <<< "$CURRENT"
  VERSION="${major}.${minor}.$((patch + 1))"
fi

# Confirm
read -rp "Release: v${CURRENT} -> v${VERSION}, continue? [Y/n] " confirm
if [[ "${confirm:-Y}" =~ ^[Nn] ]]; then
  echo "Aborted."
  exit 0
fi

# 1. Check working directory is clean
if [ -n "$(git status --porcelain)" ]; then
  echo "Error: working directory is not clean. Commit or stash changes first."
  exit 1
fi

# 2. Check we're on main branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
  echo "Error: not on main branch (on '$BRANCH')"
  exit 1
fi

# 3. Check tag doesn't already exist
if git rev-parse "v${VERSION}" >/dev/null 2>&1; then
  echo "Error: tag v${VERSION} already exists"
  exit 1
fi

# 4. Update Cargo.toml version
sed -i '' "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
echo "Updated Cargo.toml to version ${VERSION}"

# 5. Build to update Cargo.lock
cargo build --release
echo "Build successful"

# 6. Commit
git add Cargo.toml Cargo.lock
git commit -m "release: v${VERSION}"

# 7. Tag and push
git tag "v${VERSION}"
git push origin main --tags

echo ""
echo "Done! v${VERSION} released."
echo "CI will build binaries and update homebrew-tap automatically."
echo "Track progress: gh run list --repo gowcar/mdx --limit 1"
