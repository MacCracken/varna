#!/bin/sh
# Bump the project version. VERSION is the single source of truth — cyrius.cyml
# reads it via ${file:VERSION}. Also keeps the daimon registration version string
# (src/daimon.cyr) in sync.
#
# Usage: ./scripts/version-bump.sh 2.1.0
set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Current: $(cat VERSION 2>/dev/null)"
    exit 1
fi

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
NEW="$1"
OLD=$(tr -d '[:space:]' < "$REPO_ROOT/VERSION")

if [ "$NEW" = "$OLD" ]; then
    echo "Already at $OLD"
    exit 0
fi

echo "$NEW" > "$REPO_ROOT/VERSION"

# daimon registration version string — must match VERSION.
if [ -f "$REPO_ROOT/src/daimon.cyr" ]; then
    sed -i "s|store64(r + 8, \"$OLD\");|store64(r + 8, \"$NEW\");|" "$REPO_ROOT/src/daimon.cyr"
fi

echo "Bumped $OLD -> $NEW (cyrius.cyml tracks VERSION via \${file:VERSION})."
echo "Add a CHANGELOG entry, then tag and push."
