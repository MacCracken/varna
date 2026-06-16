# varna — Current State

> Refreshed every release. CLAUDE.md is preferences/process/procedures
> (durable); this file is **state** (volatile).

## Version

**2.0.0** — ported from Rust to Cyrius (2026-06-16) via `cyrius port`. 8386 lines of
Rust preserved at `rust-old/` for parity reference. See
[ADR 0001](../adr/0001-port-from-rust-to-cyrius.md).

## Toolchain

- **Cyrius pin**: `6.2.12` (in `cyrius.cyml [package].cyrius`)

## Source

- Rust reference: 8386 lines at `rust-old/` (frozen, do not edit).
- Cyrius port: **scaffold green** — `src/main.cyr` stub builds and runs. The
  module-by-module reimplementation of `src/*.cyr` (per the `[lib]` list in
  `cyrius.cyml`) is the in-flight work, at v1.0 data parity.

## Tests

- `cyrius tests` — smoke green (`tests/varna.tcyr`, 2 assertions).
- `cyrius bench tests/varna.bcyr` — harness green (`noop` benchmark).
- Parity tests against `rust-old/` land with each ported module.

## Dependencies

Direct (declared in `cyrius.cyml [deps].stdlib`):

- string, fmt, alloc, vec, str, slice, syscalls, io, args, assert, hashmap, bayan,
  fnptr, tagged, result, bench

Deferred to their `-D`-gated surfaces (added with the module that needs them):

- `-D LOGGING` → `sakshi`
- `-D MCP` → `bote` (`dist/bote-core.cyr`, tag 2.7.6) + crypto/thread companions

## Consumers

shabda, shabdakosh, svara, sankhya, jnana, vidya (planned: vansh, sahifa).

## Next

See [`roadmap.md`](roadmap.md). The current milestone is Rust→Cyrius surface parity:
reimplement each `src/*.cyr` module against the `rust-old/` oracle, keeping
`cyrius build` / `cyrius tests` green at every step.
