# Getting started with varna

## Build

```sh
cyrius deps                              # resolve dependencies
cyrius build src/main.cyr build/varna    # compile
cyrius test                              # run tests/*.tcyr
```

## Layout

- `src/main.cyr` — entry point. Top-level `var r = main(); syscall(SYS_EXIT, r);`.
- `tests/` — test suite (`.tcyr` files, auto-discovered by `cyrius test`).
- `dist/` — the bundles `cyrius distlib` builds for consumers (`varna.cyr`, `varna-core.cyr`, and their `.deps` sidecars).

## Adding a feature

1. Edit `src/main.cyr` (or add a new module and `include` it).
2. Add a test case to the matching `tests/*.tcyr` (the v1.x Rust oracle was removed at
   2.1.4 — its behaviour now lives in `tests/invariants.tcyr`, `adversarial.tcyr`,
   `integration.tcyr` and `mcp_json.tcyr`).
3. Run `sh scripts/check.sh` (deps + lock verify + fmt + lint + build + tests).
4. Bump `VERSION` and add a CHANGELOG entry before tagging.

See [`../adr/template.md`](../adr/template.md) when a non-trivial design choice deserves an ADR.
