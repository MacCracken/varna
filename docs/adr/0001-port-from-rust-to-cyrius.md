# 0001 — Port from Rust to Cyrius

**Status**: Accepted — the `rust-old/` freeze is superseded by [ADR 0002](0002-remove-the-rust-old-archive.md)
**Date**: 2026-06-16

> **Amendment (2026-08-27, v2.1.4).** This ADR froze the Rust source into `rust-old/`
> as a parity oracle. That directory was deleted at 2.1.4 once the port was verified
> complete and the four standalone Rust test suites were carried over — see
> [ADR 0002](0002-remove-the-rust-old-archive.md). References to `rust-old/` below are
> preserved as written; the crate remains recoverable from git tags `1.0.0`–`2.1.3`.

## Context

Varna shipped as a Rust crate through v1.0 — a flat library of phoneme inventories,
writing-system metadata, grammar profiles, and lexicon access for 51 languages, with
optional AI-integration surfaces (MCP, daimon, hoosh) behind Cargo features. The crate
was clean, benchmarked, and at ~98% test coverage. But varna sits at the base of an
AGNOS stack whose every other tier — its consumers (shabda, shabdakosh, svara, sankhya,
jnana, vidya) and the language they target — is moving to Cyrius, the sovereign
self-hosting systems language. A Rust data layer under a Cyrius stack is a seam: two
toolchains, two dependency graphs, a foreign-language link boundary at the exact point
where the most data crosses.

Two constraints made this a real decision rather than reflex:

1. **Stack coherence.** Varna's direct consumers are Cyrius projects. Depending on a
   Cyrius `dist/varna.cyr` bundle (folded in single-file, the established AGNOS
   DEPS-PATTERN) removes the FFI boundary entirely — phoneme data is read as native
   Cyrius structs, not marshalled across a language gap.
2. **Stdlib sufficiency.** Varna's external dependencies (`serde`, `serde_json`,
   `thiserror`, `tracing`, `tracing-subscriber`, `criterion`) all have Cyrius stdlib
   equivalents that already ship: `bayan` (JSON + `#derive(Serialize)`), native
   `enum`/`Result` (`lib/result.cyr`), `log` + `sakshi`, and `cyrius bench`. There is no
   capability gap to close before porting — the dependencies are available in the
   language.

The cost: retiring working Rust, the `criterion` statistical harness, the `cargo doc`
browsable reference, and the `serde` derive-based (de)serializer path. Real value,
deliberately frozen into `rust-old/`.

## Decision

**Port varna from Rust to Cyrius at v2.0.** Use `cyrius port` to move the Rust source to
`rust-old/` as a frozen historical artifact (per the first-party-standards `rust-old/`
convention) and scaffold the Cyrius project (`cyrius.cyml`, `src/`, docs tree, CI).
Re-implement each domain as a `src/*.cyr` module, vendor the Cyrius stdlib via
`cyrius deps`, and publish the engine as a single-file `dist/varna.cyr` bundle (plus a
transport-free `dist/varna-core.cyr`) for downstream folding.

**Scope in**:
- The full data engine: phoneme, script, grammar, lexicon, registry, dialect, and their
  submodules (allophone, syllable, transliteration, numerals, swadesh, cognate).
- All 51 language inventories, 10 scripts, 5 numeral systems, 11 grammar profiles — at
  parity with v1.0.
- JSON serialization via `bayan` `#derive(Serialize)`; deserialization hand-rolled via
  `json_parse`/`json_get` (Cyrius has no `#derive(Deserialize)`).
- The optional AI surfaces as `-D` build defines: `-D LOGGING`, `-D MCP` (folding
  `bote-core`), `-D DAIMON`, `-D HOOSH`.

**Scope out**:
- The Cargo feature system (`std`/`logging`/`mcp`/`daimon`/`hoosh`/`full`) — replaced by
  `-D` build defines; no `std`/`no_std` split.
- `criterion` statistical benchmarking — replaced by `lib/bench.cyr` via `cyrius bench`.
- `cargo doc` — the public surface is a folded `.cyr` bundle read in `src/`.
- A live link against `daimon`/`hoosh`: those Cargo features only flipped on
  `serde_json`; they never linked the crates. Both ship today as Cyrius **binaries** with
  no consumable `[lib]`/`dist` bundle, so the `-D DAIMON`/`-D HOOSH` surfaces only toggle
  JSON output (via `bayan`, already a stdlib dep). If real integration is wanted later,
  those projects must first expose a `dist` bundle (the `bote` pattern).

## Consequences

### Positive

- **Seam removed.** Consumers fold `dist/varna.cyr` and read phoneme/script/grammar data
  as native Cyrius structs — no FFI, no second toolchain in the build.
- **Zero external dependencies.** The whole stack is Cyrius stdlib + one git dep
  (`bote-core`, only under `-D MCP`). Supply-chain surface collapses to the AGNOS-owned
  set.
- **Naming cleanup.** The port retires the legacy `lipi`/`LIPI` identifiers (`LIPI_LOG`,
  `LipiError`, `lipi_*` tools) in favour of `varna`/`VARNA` — long overdue for
  project-name consistency.

### Negative

- **No `cargo doc` browsable reference.** Discovery relies on reading the `src/*.cyr`
  modules and the folded bundle. Acceptable: varna's surface is a small, stable set of
  free functions.
- **No `criterion`.** Lose statistical significance + outlier rejection; `cyrius bench`
  gates per-release perf, not per-PR regression detection.
- **Hand-rolled deserialization.** No `#derive(Deserialize)` — JSON ingestion uses
  `bayan` accessors. Acceptable: varna mostly *emits* JSON; ingestion is rare.
- **`rust-old/` is dead weight** in the tree. Frozen per convention, gitignored from CI;
  the `(migrated at v2.0)` note in CLAUDE.md and README marks it as historical.

### Neutral

- The source-level reimplementation is a follow-on step: this release lands the Cyrius
  project metadata (`cyrius.cyml`), the verified dependency mapping, and the documentation
  port. The `src/*.cyr` modules land next, module by module, against the same parity
  target.

## Alternatives considered

- **Keep the Rust crate, expose a C ABI for Cyrius consumers.** Rejected. Preserves the
  FFI seam permanently and forces every consumer to marshal structs across the boundary —
  the opposite of the stack-coherence goal.
- **Dual-stack: maintain both Rust and Cyrius implementations.** Rejected as scope creep.
  Two libraries, two test paths, two release artifacts, no deprecation horizon.
- **Wait for `#derive(Deserialize)` before porting.** Rejected. Varna's hot path is
  emitting data, not ingesting it; hand-rolled `bayan` parsing covers the rare ingestion
  case without blocking the port.
- **Rewrite in C.** Rejected. C is not the AGNOS first-party language; it would satisfy
  neither stack coherence (still an FFI boundary for Cyrius consumers) nor the
  self-hosting direction of the ecosystem.
