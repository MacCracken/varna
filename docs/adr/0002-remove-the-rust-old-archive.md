# 0002 — Remove the `rust-old/` archive

**Status**: Accepted
**Date**: 2026-08-27

Supersedes the "freeze the Rust source into `rust-old/`" half of
[ADR 0001](0001-port-from-rust-to-cyrius.md). The rest of 0001 — the decision to port,
the scope, the dependency mapping — stands.

## Context

[ADR 0001](0001-port-from-rust-to-cyrius.md) moved the v1.x Rust crate to `rust-old/`
as a frozen parity oracle: 35 files, 8,386 lines, 484K. The reason was concrete rather
than sentimental. The port was source-level reimplementation, not translation, so the
Rust source was the only reference for what each module was supposed to do. 0001 already
called the directory "dead weight in the tree" and expected it to go; it did not say
when, because at the time nobody had established what would be lost.

The 2.1.2 hardening sweep and the 2.1.3 test port together answered that. Two findings
made the removal decidable:

1. **The API port is complete.** Verified name by name, not by counting: 19 Rust modules
   map 1:1 onto 19 Cyrius modules (plus `util.cyr`, which has no Rust counterpart); all
   150 Rust `pub fn` have Cyrius equivalents; the 48 language inventories match exactly.
   Nothing under `src/` or `tests/` ever compiled against `rust-old/` — every reference
   to it was a provenance comment.

2. **The test suites were the real dependency, and they are now ported.** The 2.0.0 port
   carried each module's in-module unit tests but left the four standalone suites under
   `rust-old/tests/` behind — 147 tests. That was not a filing detail: `adversarial.rs`
   contained `registry_info_very_long_code`, `phoneme_find_very_long_string` and
   `error_display_very_long`, which is the exact class of test that would have caught
   the heap overflows 2.1.2 had to fix. Until those suites were ported, deleting the
   directory would have destroyed the record of what they asserted. 2.1.3 ported all
   four (`tests/invariants.tcyr`, `adversarial.tcyr`, `integration.tcyr`,
   `mcp_json.tcyr`), taking the suite from 652 to 1,005 assertions.

## Decision

**Delete `rust-old/` at 2.1.4.**

The directory is tracked in git, so tags `1.0.0` and `2.0.0` — and every commit through
`2.1.3` — keep the full Rust crate recoverable. Removal takes it out of the working tree
and off the reading path; it does not destroy it.

**Scope in**:
- `rust-old/` and its 35 files.
- The `/rust-old/target/` entry in `.gitignore`.
- The 43 provenance comments in `src/*.cyr` and `tests/*.{tcyr,bcyr}`, rewritten from
  `rust-old/src/X.rs` to `v1.x Rust src/X.rs` — the provenance is worth keeping, a path
  that no longer resolves is not.
- The `rust-old/` mentions in `README.md`, `SECURITY.md`, `CLAUDE.md`,
  `docs/guides/getting-started.md` and `docs/development/state.md`.

**Scope out**:
- `docs/benchmarks-rust-vs-cyrius.md` keeps its Rust criterion figures. They are the
  only surviving record of v1.x performance and the document is explicitly historical;
  it gains a note that the source tree is gone.
- Four Rust tests that could not port are documented where they would have lived, not
  deleted silently — see the 2.1.3 CHANGELOG entry.

## Consequences

- **Positive** — the tree contains one language again. A reader cloning varna no longer
  meets 8,386 lines of Rust before the 5,000 lines of Cyrius that are the actual project.
  The `cargo`/`clippy`/`rustc` prohibition in `CLAUDE.md` stops being a live hazard and
  becomes a historical note. `git archive` release tarballs shrink by 484K.
- **Negative** — the parity oracle is gone from the working tree. A future question of
  the form "what did the Rust version do here?" now needs `git show 2.1.3:rust-old/...`
  rather than a file open. This is the cost the decision accepts, and it is bounded by
  the 1,005 assertions that now encode the answers.
- **Neutral** — `docs/benchmarks-rust-vs-cyrius.md` becomes a document about a tree that
  no longer exists. Kept deliberately: the numbers are not reproducible from anything
  still present, so deleting it would lose data that the git history does not
  conveniently surface.

## Alternatives considered

- **Keep `rust-old/` indefinitely.** Rejected: 0001 already identified it as dead weight,
  and once the suites were ported it retained no function. "Harmless" is not the same as
  "useful"; it was still 484K a new reader had to be told to ignore.
- **Delete at 2.1.2, alongside the hardening work.** Rejected at the time and the
  judgement held: the 147 unported tests were discovered during that review, and removing
  the directory first would have thrown away the specification for tests not yet written.
  Port first, delete second.
- **Delete the provenance comments along with the directory.** Rejected. "Ported from
  `src/phoneme/mod.rs`" is still true and still useful for anyone reconstructing why a
  module is shaped the way it is; only the `rust-old/` prefix became false.
- **Move `rust-old/` to a separate archive repository.** Rejected as ceremony. Git tags
  already provide exactly this, at no cost and with no second repository to maintain.
