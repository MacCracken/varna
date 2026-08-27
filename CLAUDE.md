# Varna — Claude Code Instructions

## Project Identity

**Varna** (Sanskrit: वर्ण — letter, character, sound) — Multilingual language engine: phoneme inventories, writing systems, grammar profiles, and lexicon access for 50+ languages

- **Type**: Flat Cyrius library (`dist/varna.cyr` bundle) + thin demo entry — no longer a Rust crate; migrated at v2.0 (see [ADR 0001](docs/adr/0001-port-from-rust-to-cyrius.md))
- **License**: GPL-3.0
- **Toolchain**: Cyrius, pinned in `cyrius.cyml` (`cyrius = "6.5.35"`)
- **Version**: SemVer; canonical source is `VERSION` (read by `cyrius.cyml` via `${file:VERSION}`). Current target: **2.1.1**
- **Rust archive**: the frozen v1.x crate lives in `rust-old/` — a historical artifact, gitignored from CI, never edited

## Consumers

shabda (G2P), shabdakosh (pronunciation dict), svara (vocal synthesis), sankhya (ancient math systems), jnana (knowledge), vidya (programming reference), vansh (voice assistant, planned), sahifa (document processing, planned)

## Toolchain — the Cyrius surface

Varna is a single Cyrius project under `src/`, `lib/`, `tests/`. Build, test,
lint, and bench with the cyrius toolchain only.

| Action | Command | Notes |
|---|---|---|
| Resolve deps | `cyrius deps` | Reads `cyrius.cyml`, copies stdlib into `lib/` (gitignored); bote is added under `-D MCP` |
| Verify lock | `cyrius deps --verify` | Checks `cyrius.lock` hashes; part of `scripts/check.sh` |
| Build | `cyrius build src/main.cyr build/varna` | `-D LOGGING -D MCP -D DAIMON -D HOOSH` for optional surfaces |
| Bundle | `cyrius distlib` | Concatenates `src/` → `dist/varna.cyr` for consumers (`distlib core` → `dist/varna-core.cyr`) |
| Run tests | `cyrius tests` | Recursively runs `tests/*.tcyr` |
| Run benchmarks | `cyrius bench tests/varna.bcyr` | `lib/bench.cyr` harness (`.bcyr` in `tests/`) |
| Format | `cyrius fmt <file> [--check]` | Per-file; `--check` is the CI gate |
| Lint | `cyrius lint <file>` | Static analysis (`#skip-lint` exempts a line) |
| Vet deps | `cyrius vet src/main.cyr` | Audit include dependencies |
| Enforce policy | `cyrius deny src/main.cyr` | Deny-list checks |
| Doc | `cyrius doc --check src/main.cyr` | Doc currency gate |
| Local gate | `sh scripts/check.sh` | deps + lock verify + fmt + lint + build (default + full) + tests (`cyrius audit` needs the cyrius-repo check.sh, not installed for consumers) |

**Never run `cargo`, `clippy`, `rustc`, `cargo-audit`, or `cargo-deny`** — those are
stale references from the pre-2.0 Rust era. They survive only in `rust-old/`, which is
frozen.

## Development Process

### P(-1): Scaffold Hardening (before any new features)

0. Read roadmap, CHANGELOG, and open issues — know what was intended before auditing what was built
1. Test + benchmark sweep of existing code (`cyrius tests`, `cyrius bench`)
2. Cleanliness check: `cyrius fmt --check`, `cyrius lint`, `cyrius vet`, `cyrius deny`, `cyrius doc --check` (or `sh scripts/check.sh`)
3. Get baseline benchmarks (`./scripts/bench-history.sh`)
4. Internal deep review — gaps, optimizations, security, logging/errors, docs
5. External research — domain completeness, missing capabilities, best practices, world-class accuracy
6. Cleanliness check — must be clean after review
7. Additional tests/benchmarks from findings
8. Post-review benchmarks — prove the wins
9. Repeat if heavy

### Work Loop / Working Loop (continuous)

1. Work phase — new features, roadmap items, bug fixes
2. Cleanliness check: `cyrius fmt --check`, `cyrius lint`, `cyrius vet`, `cyrius deny`, `cyrius doc --check` (or `sh scripts/check.sh`)
3. Test + benchmark additions for new code
4. Run benchmarks (`./scripts/bench-history.sh`)
5. Internal review — performance, memory, security, throughput, correctness
6. Cleanliness check — must be clean after audit
7. Deeper tests/benchmarks from audit observations
8. Run benchmarks again — prove the wins
9. If audit heavy → return to step 5
10. Documentation — update CHANGELOG, roadmap, docs
11. Version check — `VERSION` and `cyrius.cyml` (via `${file:VERSION}`) and the zugot recipe all in sync
12. Return to step 1

### Task Sizing

- **Low/Medium effort**: Batch freely — multiple items per work loop cycle
- **Large effort**: Small bites only — break into sub-tasks, verify each before moving to the next. Never batch large items together
- **If unsure**: Treat it as large. Smaller bites are always safer than overcommitting

### Refactoring

- Refactor when the code tells you to — duplication, unclear boundaries, performance bottlenecks
- Never refactor speculatively. Wait for the third instance before extracting an abstraction
- Refactoring is part of the work loop, not a separate phase. If a review (step 5) reveals structural issues, refactor before moving to step 6
- Every refactor must pass the same cleanliness + benchmark gates as new code

### Key Principles

- **Never skip benchmarks.** Numbers don't lie. The CSV history is the proof.
- **Tests + benchmarks are the way.** Minimum 80%+ coverage target.
- **Own the stack.** If an AGNOS module wraps a capability, depend on the AGNOS Cyrius distfile (`lib/*.cyr`), not an external lib.
- **No magic.** Every operation is measurable, auditable, traceable.
- **Tagged enums** for all public sum types — variants are additive, so new variants don't break consumers (the Cyrius equivalent of `#[non_exhaustive]`).
- **`#must_use`** on pure functions.
- **Let the optimizer inline.** Cyrius runs an O1–O6 pass with linear-scan regalloc; don't hand-annotate hot paths.
- **Write to buffers, not temporaries** — the bump allocator never frees, so avoid throwaway allocations (`fmt`/`io` into a reused buffer).
- **Borrow static `str` literals**; allocate via `lib/str.cyr` only when you must (Cyrius has no `Cow`).
- **`-D` define optional surfaces** — consumers compile only what they pass.
- **Structured logging on all operations** — `lib/log.cyr` → `lib/sakshi.cyr` for the audit trail (under `-D LOGGING`).

## DO NOT
- **Do not commit or push** — the user handles all git operations (commit, push, tag)
- **NEVER use `gh` CLI** — use `curl` to GitHub API only
- **Never run `cargo` / `clippy` / `rustc` / `cargo-audit` / `cargo-deny`** against the project — varna migrated off Rust at v2.0; those tools apply only to the frozen `rust-old/`
- Do not add unnecessary dependencies — keep it lean
- Do not introduce implicit aborts — there is no `unwrap()`/`panic!()`; check sentinel/`Result` returns from every fallible call
- Do not skip benchmarks before claiming performance improvements
- Do not commit `build/` or the vendored `lib/` (re-resolved by `cyrius deps`); `cyrius.lock` **is** committed (pins the resolved deps)

## Documentation Structure

```
Root files (required):
  README.md          — quick start, build defines, dependency stack, consumers, license
  CHANGELOG.md       — per-version changes (Added/Changed/Fixed/Removed/Performance)
  CLAUDE.md          — this file (development process, principles, DO NOTs)
  CONTRIBUTING.md    — fork, branch, cyrius gates, PR workflow
  SECURITY.md        — supported versions, scope, reporting
  CODE_OF_CONDUCT.md — Contributor Covenant
  LICENSE            — GPL-3.0
  cyrius.cyml        — package + build + [lib]/[deps] manifest

docs/ (required):
  architecture/
    overview.md      — module map, data flow, consumers, dependency stack
  development/
    roadmap.md       — completed items, backlog, future features, v-criteria

docs/ (when earned — not scaffolded empty):
  adr/
    NNN-title.md     — architectural decision records (0001 = the Rust→Cyrius port)
  guides/
    usage.md         — patterns, philosophy, code examples
```

## CHANGELOG Format

Follow [Keep a Changelog](https://keepachangelog.com/):

```markdown
# Changelog

## [Unreleased]
### Added — new features
### Changed — changes to existing features
### Fixed — bug fixes
### Removed — removed features
### Performance — benchmark-proven improvements (include numbers)

## [X.Y.Z] - YYYY-MM-DD
### Added
- **module_name** — what was added and why
```

Rules:
- Every PR/commit that changes behavior gets a CHANGELOG entry
- Performance claims MUST include benchmark numbers (from `cyrius bench`)
- Breaking changes get a **Breaking** section with migration guide
- Group by module when multiple changes in one release
