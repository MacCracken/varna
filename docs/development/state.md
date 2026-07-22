# varna — Current State

> Refreshed every release. CLAUDE.md is preferences/process/procedures
> (durable); this file is **state** (volatile).

## Version

**2.1.0** — toolchain-maintenance release (2026-07-21): Cyrius pin `6.2.12` → `6.4.69`,
vendored stdlib refreshed, `cyrius.lock` regenerated. No API or data changes.
Originally ported from Rust to Cyrius at **2.0.0** (2026-06-16) via `cyrius port`; 8386
lines of Rust preserved at `rust-old/` for parity reference. See
[ADR 0001](../adr/0001-port-from-rust-to-cyrius.md).

## Toolchain

- **Cyrius pin**: `6.4.69` (in `cyrius.cyml [package].cyrius`)

## Source

- Rust reference: 8386 lines at `rust-old/` (frozen, do not edit).
- Cyrius port (in-flight, module by module against the `rust-old/` oracle):
  - ✅ `src/error.cyr` — VarnaError codes
  - ✅ `src/phoneme.cyr` — phoneme types, builder, `english`/`sanskrit`/`greek`
  - ✅ `src/inventories.cyr` — 48 extended language inventories (51 languages total)
  - ✅ `src/registry.cyr` — ISO 639 lookup: `info`/`phonemes`/`all_codes`/`primary_script[_code]`
  - ✅ `src/script.cyr` — 10 writing systems (type/direction/status/Unicode ranges,
    `by_code`/`contains_codepoint`); unblocked `registry_primary_script`
  - ✅ `src/numerals.cyr` — 5 numeral systems (Deva digits, Greek isopsephy, Babylonian,
    Egyptian, Chinese rod); UTF-8 `string_value`
  - ✅ `src/transliteration.cyr` — Devanagari↔IAST, Greek↔Beta Code (greedy longest-match, reverse map)
  - ✅ `src/util.cyr` — shared `_utf8_len` codepoint helper
  - ✅ `src/allophone.cyr` — English allophone rules (flapping/aspiration/dark-l); `realize`/`rules_for`
  - ✅ `src/syllable.cyr` — syllable templates + phonotactics (English/Sanskrit/Japanese); tri-state `is_permitted`
  - ✅ `src/grammar.cyr` — 11 typological profiles (morphology/word-order/case/gender/classifiers); `by_code`
  - ✅ `src/lexicon.cyr` — LexEntry/Lexicon (`find`/`swadesh`/`most_frequent` with sort), PartOfSpeech
  - ✅ `src/swadesh.cyr` — Swadesh-25 lists for 10 languages (250 entries)
  - ✅ `src/cognate.cyr` — water cognates + CognateSet/Etymology/BorrowingType
  - ✅ `src/dialect.cyr` — variety overlays (British English RP); `adds`/`removes`/`apply`
  - **Core data engine complete — 15 modules.** `cyrius distlib` → `dist/varna.cyr` bundles + compiles.
  - ✅ `src/daimon.cyr` (`-D DAIMON`) — agent registration (6 capabilities, 51 langs, 10 scripts)
  - ✅ `src/hoosh.cyr` (`-D HOOSH`) — `LanguageQuery` + `answer_from_data` (string-built content;
    confidence per-mille, structured_data deferred)
  - ✅ `src/logging.cyr` (`-D LOGGING`) — level-gated stderr logger (`io`); env-filter/`sakshi` deferred
  - ✅ `src/mcp.cyr` (`-D MCP`) — 5 tools + `invoke`; lightweight ToolDef + hand-built JSON (no bote/bayan)
  - **Port complete — 19 modules (15 core + 4 `-D` surfaces).** Default `cyrius build` excludes
    surfaces (973 unreachable); `-D LOGGING -D MCP -D DAIMON -D HOOSH` includes all (1023).
    `cyrius distlib` → `dist/varna.cyr`. Gating: `#ifdef NAME` body; each test `#define`s its
    macro so `cyrius tests` exercises it.

## Tests

- `cyrius tests` — green: 526 parity assertions + smoke — phoneme (32) + inventories (159) +
  registry (15) + script (37) + numerals (34) + transliteration (14) + allophone (8) + syllable (21) +
  grammar (47) + swadesh (78) + cognate (13) + dialect (11) + daimon (9) + hoosh (13) +
  logging (5) + mcp (30) + `tests/varna.tcyr` (smoke).
- Full parity audit (7-agent workflow, 2026-06-16): **0 unported public functions**; every
  divergence documented (sentinels-for-Option, tagged enums, hand-built JSON, omitted
  name-colliding enum variants). One fidelity fix applied (mcp compare `unique_to_*` fields).
- `sh scripts/check.sh` — local gate green: `cyrius deps` + `fmt --check` + `lint` (0 warnings)
  + build (default + `-D` full) + `cyrius tests`. Also `cyrius vet`/`deny`/`doc --check` pass.
- `cyrius bench tests/varna.bcyr` / `./scripts/bench-history.sh` — 18 benchmarks baselined
  (`bench-history.csv` + `BENCHMARKS.md`); covers every domain.
- **Release-ready (2.1.0):** version synced (`VERSION` / `cyrius.cyml` `${file:VERSION}` /
  daimon string / `CHANGELOG [2.1.0]`); `cyrius.lock` regenerated at 6.4.69 and
  `cyrius deps --verify` clean (29 verified, 0 failed); CI runs `check.sh` + bench + distlib;
  release.yml bundles `dist/varna.cyr`.

## Dependencies

Direct (declared in `cyrius.cyml [deps].stdlib`):

- string, fmt, alloc, vec, str, slice, syscalls, io, args, assert, hashmap, bayan,
  fnptr, tagged, result, bench

Deferred to their `-D`-gated surfaces (planned, not yet wired — the surfaces
currently self-contain, so these are absent from `cyrius.lock` until a module
actually resolves them):

- `-D LOGGING` → `sakshi` (logging.cyr is a self-contained level logger today)
- `-D MCP` → `bote` (`dist/bote-core.cyr`, tag 2.7.6) + crypto/thread companions
  (mcp.cyr hand-builds JSON today, no bote/bayan)

## Consumers

shabda, shabdakosh, svara, sankhya, jnana, vidya (planned: vansh, sahifa).

## Next

The port shipped as 2.0.0; 2.1.0 is a toolchain-maintenance release (Cyrius 6.4.69).
Remaining work is post-port:

- **2.0.1 — remove `rust-old/`** (planned, separate session, after another review
  sweep). `rust-old/` is currently kept as the frozen parity oracle per
  [ADR 0001](../adr/0001-port-from-rust-to-cyrius.md). When it's removed, sweep the
  references that point into it:
  - ADR 0001 — supersede/annotate the "preserve `rust-old/`" decision
  - CHANGELOG `[2.0.0]`, SECURITY.md (`frozen in rust-old/`), CLAUDE.md, this file, roadmap
  - module headers (`# Ported from rust-old/src/...`) across `src/*.cyr`
  - `docs/benchmarks-rust-vs-cyrius.md` bottom pointer; `.gitignore` `/rust-old/target/`
  - the Rust criterion numbers are already preserved in `docs/benchmarks-rust-vs-cyrius.md`,
    so removal loses no benchmark history
- **Optimization** (unstarted, see `docs/benchmarks-rust-vs-cyrius.md`): cache the immutable
  pre-built inventories/scripts/profiles (build-once); intern lookups. Parity-safe wins.
