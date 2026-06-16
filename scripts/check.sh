#!/bin/sh
# Local cleanliness + test gate for varna. Run before committing.
#
# `cyrius audit` expects the cyrius-repo's own check.sh (not installed for
# consumer projects), so this is varna's equivalent: fmt + lint + build
# (default and full) + tests.
set -e

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

echo "== deps =="
cyrius deps >/dev/null

echo "== fmt --check =="
for f in src/*.cyr tests/*.tcyr tests/*.bcyr; do
    cyrius fmt "$f" --check >/dev/null 2>&1 || { echo "FAIL fmt: $f"; exit 1; }
done

echo "== lint =="
for f in src/*.cyr; do
    out=$(cyrius lint "$f" 2>&1 || true)
    if printf '%s' "$out" | grep -qE '[1-9][0-9]* warning'; then
        echo "FAIL lint: $f"; printf '%s\n' "$out"; exit 1
    fi
done

echo "== build (default) =="
mkdir -p build
cyrius build src/main.cyr build/varna >/dev/null

echo "== build (-D LOGGING -D MCP -D DAIMON -D HOOSH) =="
cyrius build -D LOGGING -D MCP -D DAIMON -D HOOSH src/main.cyr build/varna-full >/dev/null

echo "== tests =="
cyrius tests

echo ""
echo "OK: all gates pass"
