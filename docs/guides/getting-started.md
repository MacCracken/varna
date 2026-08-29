# Getting started with varna

## Build

```sh
cyrius deps                              # resolve dependencies
cyrius build src/main.cyr build/varna    # compile
cyrius tests                             # run tests/*.tcyr (recursive)
```

## Layout

- `src/main.cyr` — entry point. Top-level `var r = main(); syscall(SYS_EXIT, r);`.
- `tests/` — test suite (`.tcyr` files, auto-discovered by `cyrius tests`).
- `dist/` — the bundles `cyrius distlib` builds for consumers (`varna.cyr`, `varna-core.cyr`, and their `.deps` sidecars).

## Adding a feature

1. Edit the module it belongs in, or add `src/<name>.cyr` and `include` it from
   `src/main.cyr`. **A new module costs an include line in every test file** that
   needs it — there is no transitive include — so expect to touch ~19 of them.
2. Add a test case to the matching `tests/*.tcyr` (the v1.x Rust oracle was removed at
   2.1.4 — its behaviour now lives in `tests/invariants.tcyr`, `adversarial.tcyr`,
   `integration.tcyr` and `mcp_json.tcyr`).
3. **Check the test catches the bug.** Reintroduce the defect and confirm the new
   assertion fails. A test that passes both before and after has not tested anything;
   several releases have shipped assertions that pinned the defect rather than the fix.
4. Run `sh scripts/check.sh` (deps + lock verify + fmt + lint + build + tests).
5. If you touched a hot path, benchmark old and new **interleaved in one session** and
   report spreads alongside minima. Min-of-N across separate sessions produced phantom
   regressions in four consecutive releases; overlapping spreads mean no result.
6. Bump `VERSION` and add a CHANGELOG entry before tagging.

See [`../adr/template.md`](../adr/template.md) when a non-trivial design choice deserves an ADR.
