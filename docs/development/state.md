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
