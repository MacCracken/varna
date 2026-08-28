# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.1.6] - 2026-08-27

Roadmap item 2.1.6: define the nine scripts the registry had been naming without
them. This empties the `2.1.x` carry-over list.

### Added

- **script** — nine ISO 15924 scripts that the registry referenced but `script.cyr`
  never defined, so `registry_primary_script` silently returned 0 for nine of the 51
  languages: **Thai** (`Thai`), **Bengali** (`Beng`), **Tamil** (`Taml`),
  **Ethiopic** (`Ethi`), **Hebrew** (`Hebr`), **Georgian** (`Geor`), **Myanmar**
  (`Mymr`), **Khmer** (`Khmr`) and **Lao** (`Laoo`). Each carries its type,
  direction, status, Unicode blocks and languages, and is reachable through
  `script_by_code` and `script_all_codes`. The registry now resolves a primary script
  for all 51 languages; `script_all_codes()` returns 19, up from 10.

  Blocks follow Unicode, main block first, including the supplementary ones a
  single-range definition would have missed: Tamil Supplement (`11FC0`–`11FFF`),
  Ethiopic Supplement / Extended / Extended-A, Hebrew presentation forms
  (`FB1D`–`FB4F`), Georgian Extended and Supplement, Myanmar Extended-A and -B, and
  Khmer Symbols. Hebrew is the only right-to-left addition; Georgian is the only
  alphabet, the other seven are abugidas.

- **tests/script.tcyr** — a `scripts_added_at_2_1_6` group (52 assertions) covering
  each new script's metadata, dispatch reachability, and codepoint boundaries on both
  sides of every block — including the adjacent-block confusions the ranges invite
  (Thai vs Lao at `0E7F`/`0E80`, Bengali vs Devanagari at `097F`).

### Changed

- **tests/invariants.tcyr** — `registry_script_codes_resolve` is **strict** again.
  The Rust original was deliberately tolerant ("Some scripts may not be registered yet
  (Thai, Beng, etc.)") and the 2.1.3 port carried that allowance, pinning the gap at
  exactly 9 unresolved codes. With the nine defined, the assertion now requires every
  registered language to resolve a primary script — a new language whose script is
  undefined fails here rather than returning 0 at runtime.
- **tests/integration.tcyr** — `registry_script_consistency` now expects all 51
  languages to resolve, up from 42.
- **tests/daimon.tcyr** — the agent registration advertises 19 supported scripts.

### Notes

- No performance change: `script_by_code` grew nine `streq` branches on a path that
  has been a cached singleton lookup since 2.1.5, and `script_by_code_lookup` is flat.
- Suite totals: 1,066 → 1,120 assertions across 26 test files.


## [2.1.5] - 2026-08-27

Roadmap item 2.1.5: the three items deferred out of the 2.1.2 hardening sweep.
Includes one **breaking** change to the `-D MCP` surface.

### Breaking

- **mcp** — `varna_translate_ipa` now returns a JSON object,
  `{"scheme":...,"input":...,"output":...}`, where it previously returned the
  transliterated text as a bare string. The other four tools already returned JSON
  objects, so a consumer parsing every success payload got malformed data from
  exactly one tool.

  **Migration**: read the `"output"` field instead of using the payload directly.
  `mcp_result_payload(r)` → parse → `output`.

  The contract is now stated at the top of `src/mcp.cyr` and pinned by
  `tests/mcp_json.tcyr`: **success payloads are JSON objects for all five tools;
  error payloads are plain diagnostic messages, not JSON.** Errors stay unstructured
  deliberately — the result is already tagged, so a caller never has to parse one to
  discover it is an error.

### Changed

- **Pre-built data constructors are now shared singletons.** All 98 zero-argument
  constructors — 51 `phoneme_*` inventories, 10 `script_*`, 11 `grammar_*`,
  10 `swadesh_*`, 2 `translit_*`, 5 `numerals_*`, plus `allophone_english`,
  the three `phonotactics_*`, `dialect_british_english`, `cognate_water` and the
  four `*_all_codes` list builders — build once and return the same pointer to
  every later caller.

  **This is a public-API semantic change.** Do not mutate a returned inventory
  through the `phoneme_builder_*` functions: you would be editing what every other
  caller sees. Use the new `phoneme_clone` for a mutable copy. `registry_all_codes`
  took this same change at 2.1.2; the contract is now uniform and documented at the
  head of the pre-built section in `src/phoneme.cyr`.

- **mcp** — every value written inside a JSON string literal now goes through
  `_mcp_append_json`, which escapes `"`, `\`, the five short-form control escapes
  and any other C0 byte as `\u00XX` (RFC 8259). Nothing in the current data needs
  an escape, which was exactly the problem: the invariant was unenforced and one new
  field away from emitting malformed JSON.

### Added

- **phoneme** — `phoneme_clone(inv)` returns an independent inventory that is safe
  to mutate. The copy is shallow in the same sense `dialect_apply` has always been:
  the vec, stress and tone slots are fresh, the immutable Phoneme records are shared.
- **tests/caching.tcyr** (54 assertions) — pins the singleton contract: pointer
  identity across repeat calls and through `registry_phonemes` / `script_by_code` /
  `grammar_by_code` / `swadesh_by_code` / `registry_primary_script`, that distinct
  constructors stay distinct objects, that the data is unchanged and does not
  accumulate on re-read, that `phoneme_clone` isolates mutations from the shared
  original, and that `dialect_apply` still leaves its parent untouched now that the
  parent is shared.
- **tests/mcp_json.tcyr** — a `json_escaping` group feeding a quote, a backslash, a
  newline, a tab and a raw `0x01` through `varna_translate_ipa` and checking the
  payload is still valid JSON with each byte escaped correctly.

### Performance

Interleaved A/B in one session (`cyrius bench`, min ns):

- **english_phoneme_inventory** 1,252 → 0 ns, **sanskrit** 1,664 → 6 ns, **greek**
  858 → 6 ns. The 0 is below the harness floor rather than literally free — the call
  is now a load and a compare, which the optimizer can hoist out of the loop.
- **registry_phonemes_lookup** 1,712 → 52 ns (**-97%**);
  **swadesh_by_code_lookup** 1,688 → 110 ns (**-93%**);
  **script_by_code_lookup** 287 → 102 ns (**-65%**).
- **numeral_value_of_char** 302 → 198 ns. Not attributable — the benchmark pre-builds
  its table, so nothing in this change should touch it; most likely allocation
  locality. Reported, not claimed.
- All other rows flat within noise.

**The leak is gone.** Measured directly off the bump allocator's `_heap_used`
counter over 1,000 warm calls, bytes handed out per call:

| call | before | after |
|---|---|---|
| `registry_phonemes("en")` | 2,400 B | 0 B |
| `phoneme_english()` | 2,400 B | 0 B |
| `swadesh_by_code("es")` | 1,624 B | 0 B |
| `script_by_code("Deva")` | 400 B | 0 B |

The allocator never frees, so those were permanent: a consumer polling the registry
once a second leaked ~200 MB a day.


## [2.1.4] - 2026-08-27

Roadmap item 2.1.4: remove the frozen v1.x Rust crate. The tree is single-language
again. No source behaviour changed — the only edits to `src/` are comment rewrites —
so benchmarks are unmoved and no new `bench-history.csv` row was recorded.

### Removed

- **`rust-old/`** — the frozen v1.x Rust crate: 35 files, 8,386 lines, 484K. Deleted per
  [ADR 0002](docs/adr/0002-remove-the-rust-old-archive.md), which supersedes the
  "freeze the Rust source" half of [ADR 0001](docs/adr/0001-port-from-rust-to-cyrius.md).

  It was kept as the parity oracle because the 2.0.0 port was source-level
  reimplementation, not translation. Two things made removal decidable: the API port was
  verified complete at 2.1.3 (19 modules 1:1, 150/150 `pub fn`, 48/48 inventories matching
  by name, and nothing under `src/` or `tests/` ever compiled against it), and the four
  standalone suites under `rust-old/tests/` — the real dependency, 147 tests — were ported
  in the same release. Recoverable from git tags `1.0.0`–`2.1.3`:
  `git show 2.1.3:rust-old/src/lib.rs`. Recoverability was checked against tags `2.0.0`
  and `2.1.3` before deleting.
- **`.gitignore`** — the `/rust-old/target/` entry, and the stale "Cyrius port" heading
  above it.

### Changed

- **Provenance comments** — 43 references across 42 files in `src/` and `tests/` rewritten
  from `rust-old/src/X.rs` to `v1.x Rust src/X.rs`. The provenance is worth keeping; a path
  that no longer resolves is not. Three bare-form references (`rust-old/` with no file,
  `rust-old inventory_test!`, `rust-old mod.rs sample_lexicon`) were reworded individually
  rather than pattern-matched.
- **docs/adr/0001-port-from-rust-to-cyrius.md** — status amended to note the supersession,
  with a dated amendment block. The body is left as written: an ADR records what was
  decided at the time, so its `rust-old/` references stay.
- **README.md**, **SECURITY.md**, **CLAUDE.md**, **docs/guides/getting-started.md**,
  **docs/development/state.md** — `rust-old/` mentions replaced with the tag-recovery
  route. `getting-started.md` lost its "cross-check parity against `rust-old/`" step
  (the oracle it named is gone; the behaviour now lives in the ported suites) and gained
  `dist/` in the layout list, which was never described.
- **docs/benchmarks-rust-vs-cyrius.md** — **kept**, deliberately. Its criterion figures
  are the only surviving record of v1.x performance and are not reproducible from anything
  still in the tree. Annotated with where the source went and how to recover it.

### Notes

- `git rm` was used, so the 35 deletions are staged. Nothing is committed.
- The "93 references across 49 files" figure quoted in the 2.1.3 roadmap counted the
  generated `dist/*.cyr` bundles alongside their sources. The real hand-edited count was
  43 across 42 files; the bundles picked up the rewrite when `cyrius distlib` regenerated.


## [2.1.3] - 2026-08-27

Roadmap item 2.1.3: port the four standalone Rust test suites the 2.0.0 port left
behind. Test-only — no source changes, so benchmarks are unmoved and no new
`bench-history.csv` row was recorded.

The 2.0.0 port carried every module's public API and its in-module unit tests, but
not the 147 tests under `rust-old/tests/`. That gap is why `rust-old/` could not be
deleted: removing it would have destroyed the record of what those tests asserted.
It is now closed, which unblocks 2.1.4.

### Added

- **tests/invariants.tcyr** (39 assertions) — port of `rust-old/tests/invariants.rs`
  (33 tests). Structural guarantees across the whole dataset: code uniqueness for
  languages/scripts/grammar/Swadesh, consonant+vowel totals, no duplicate or empty
  IPA symbols, inventory codes matching their registry key, tones and `Tonal` stress
  implying each other, well-formed Unicode ranges, 25-entry Swadesh lists covering
  indices 1-25, numeral tables without duplicates, decimal digit runs, and
  registered dialect parents and cognate languages.
- **tests/adversarial.tcyr** (74 assertions) — port of `rust-old/tests/adversarial.rs`
  (54 tests): empty strings, 1000-character codes, emoji, BOM, zero-width space,
  IPA lookalikes (`S` vs `ʃ`, ASCII `g` vs `ɡ`), case-sensitive ISO 15924 codes,
  codepoint boundaries either side of every script range, `u32::MAX`, empty and
  zero-capacity builders, `most_frequent(0)` and `most_frequent(1_000_000)`,
  unmapped and mixed-validity numeral strings, and dialect idempotence. **This is
  the suite whose absence let the 2.1.2 heap overflows survive** — it already
  contained `registry_info_very_long_code`, `phoneme_find_very_long_string` and
  `error_display_very_long`.
- **tests/integration.tcyr** (35 assertions) — the 18 behavioural tests from
  `rust-old/tests/integration.rs`: phoneme-kind classification, constructor field
  round-trips, registry/script consistency, the allophone/inventory boundary
  (`/t/` is an English phoneme, its intervocalic tap `[ɾ]` is not), Greek isopsephy
  of θεος, the RP overlay, cognate/registry cross-checks, Swadesh index 23 glossing
  as "water" in every list, and the tonal/non-tonal split.
- **tests/mcp_json.tcyr** (28 assertions) — stands in for
  `rust-old/tests/serde_roundtrip.rs` (27 tests), which cannot port directly: there
  is no serde in Cyrius and no deserializer to round-trip against. It checks the
  property the roundtrips were protecting — that every payload the library emits is
  structurally valid JSON — across 51 phoneme payloads, every script and grammar
  payload, and 204 compare payloads, using a structural validator that is itself
  self-checked against eleven malformed inputs.

### Changed

- **docs/development/roadmap.md** — the pre-port `1.1.0`-`1.5.0` tiers are
  renumbered `2.2.0`-`2.6.0` and the section retitled; the old numbers predated the
  2.0.0 port and no longer tracked the shipping version. 2.1.3 is removed from the
  carry-over list and 2.1.4 (`rust-old/` removal) is no longer gated.
- **docs/development/roadmap.md** — reduced to open work only. The `Completed`
  section (2.1.2 back through the 0.x Rust-crate line) is dropped; release history
  lives in this file and the git tags. Added a `2.1.x — Carry-over` section for
  work identified but not taken: 2.1.3 port the Rust integration suites, 2.1.4
  remove `rust-old/` (gated on 2.1.3), 2.1.5 the deferments from the 2.1.2
  hardening sweep, 2.1.6 script-registry completeness. The retired `v1.0 Criteria`
  checklist left one unmet item (shabda/shabdakosh consumption), now under
  `Downstream`; its "full JSON roundtrip tests for all public types" line was
  inaccurate post-port and is corrected there.
- **docs/development/state.md**, **CLAUDE.md** — record the port-completeness
  verdict, and correct CLAUDE.md's claim that `rust-old/` is "gitignored from CI"
  (it is tracked; only `rust-old/target/` is ignored).

### Verified

- **Rust → Cyrius port is complete at the API level.** 19 Rust modules map 1:1 to
  19 Cyrius modules (plus `util.cyr`); all 150 Rust `pub fn` have Cyrius
  counterparts, checked name by name; the 48 language inventories match exactly.
  Nothing in `src/` or `tests/` compiles against `rust-old/` — the 93 references
  across 49 files are provenance comments.
- **The gap is test coverage, not code.** The four suites under `rust-old/tests/`
  — `invariants.rs` (33), `adversarial.rs` (54), `integration.rs` (33),
  `serde_roundtrip.rs` (27) — were never ported. `adversarial.rs` already contains
  `registry_info_very_long_code`, `phoneme_find_very_long_string` and
  `error_display_very_long`: the class of test that would have caught the 2.1.2
  heap overflows. `rust-old/` is therefore **not yet safe to delete** — porting
  those suites first is scheduled as 2.1.3.
- **26 of 27 checkable Rust invariants hold** against current Cyrius data
  (uniqueness, consonant+vowel totals, no duplicate/empty IPA symbols, well-formed
  Unicode ranges, 25-entry Swadesh lists with indices 1-25, numeral tables,
  registered dialect parents and cognate languages). The 27th,
  `registry_script_codes_resolve`, finds 9 languages whose primary script is
  unregistered — **not a port regression**: the Rust invariant explicitly tolerated
  the same gap ("Some scripts may not be registered yet (Thai, Beng, etc.)").
  Tracked as 2.1.6.


### Notes

- **Four Rust tests do not port, and are documented inline rather than dropped
  silently.** `registry_info("en\0")` and `transliterate("\0")` tested that a Rust
  `String` may carry an interior NUL; a Cyrius cstring ends at the first NUL, so the
  property does not exist here. The three `error_display_*` tests formatted a
  payload into `VarnaError`, which in Cyrius is a plain integer code with static
  messages — the concern behind them (an unbounded caller string reaching a
  formatter) lives in `_tool_err2` and is covered by `tests/hardening_surfaces.tcyr`.
- **All 26 checkable invariants hold.** The 27th, `registry_script_codes_resolve`,
  is ported in the tolerant form the Rust original used and pins the current gap
  exactly: 9 of 51 languages name an unregistered primary script (Thai, Bengali,
  Tamil, Amharic, Hebrew, Georgian, Burmese, Khmer, Lao). Not a port regression —
  the Rust invariant carried the same allowance in a comment. Roadmap 2.1.6 adds the
  scripts, at which point the assertion can be tightened.
- Suite totals: 21 → 25 test files, 652 → 1,005 assertions, reference coverage
  89% → 94%.


## [2.1.2] - 2026-08-27

Hardening release from a P(-1) scaffold sweep. Six memory-safety defects fixed —
two of them heap overflows reachable from the `-D MCP` and `-D HOOSH` surfaces
with caller-controlled length — plus the allocation-churn and lookup-complexity
work the audit turned up. No public API or linguistic data changed.

### Security

- **mcp** — `_tool_err2` rendered `"<prefix><value>"` into a fixed `alloc(256)`
  with no bound on `value`, which is an unvalidated tool parameter. A 400-byte
  `language` / `code` / `scheme` / `lang1` / tool name wrote ~418 bytes into the
  256-byte allocation — a heap overflow with both length and contents chosen by
  the caller. Reachable from all five tools plus the unknown-tool path
  (6 call sites). Every append in the file now takes the buffer capacity and
  truncates; `tests/hardening_surfaces.tcyr` fails on the pre-fix code at all six.
- **hoosh** — `hoosh_answer_from_data` interpolated the caller's `ipa` string into
  a fixed `alloc(256)`. On the not-in-inventory branch that string is never matched
  against anything first, so its length was entirely the caller's choice — the same
  overflow. Bounded the same way.
- **transliteration** — `translit_apply` walked the input with `_utf8_len`, which
  classifies a lead byte without checking that its continuation bytes are present.
  A truncated codepoint at the tail (a lone `0xF0`) advanced the cursor 4 bytes
  with 1 byte left, so the following `memcpy` read up to 3 bytes past the end of
  the input. Reachable through the `varna_translate_ipa` MCP tool.
- **numerals** — `numerals_string_value` had the identical over-read, and is
  reachable through daimon's `numeral_value` capability.
- **util** — new `_utf8_step(s, p, n)` clamps every stride to the bytes actually
  present and never returns 0; it is now the only way the tree walks UTF-8.
  `_utf8_len` is kept and documented as lead-byte classification only.
- **transliteration** — `translit_apply` sized its output at `inlen * 4`, which
  held only because both bundled tables happen to contract. A table mapping a
  short source to a longer target overflowed the buffer, and nothing
  bounds-checked the writes. Output is now sized from the table's longest target
  (`_translit_max_target`, memoised and invalidated by `_tt_add`) and every write
  is checked against the capacity regardless.

### Changed

- **registry** — `registry_all_codes` caches its vec instead of rebuilding a
  51-element one per call. **The result is now shared and must be treated as
  read-only.** The old behaviour leaked a vec per call, since the bump allocator
  never frees.
- **phoneme** — documented that `phoneme_builder_with_capacity`'s `cap` argument
  is ignored: `lib/vec.cyr` exposes no capacity-taking constructor. Kept in the
  signature because the bundle is a published consumer API.

### Added

- **tests** — `tests/hardening.tcyr` (26 assertions) and
  `tests/hardening_surfaces.tcyr` (28) pin every fix above. Both were checked
  against the pre-fix code: the surface tests fail 7 assertions and the
  expanding-table test fails 2, so they are regression guards rather than
  restatements of current behaviour.
- **tests** — `registry_table_consistency` ties `registry_phonemes` to
  `_registry_build`. The two list the same 51 codes in different forms with
  nothing connecting them, so a language could be registered with no inventory
  and only surface downstream.
- **tests** — coverage sweep over the accessor surface `cyrius coverage` reported as
  unreferenced. Reference coverage 204/278 fns (73%) → 249/278 (89%), clearing the 80%
  floor in CLAUDE.md; `swadesh`, `syllable`, `lexicon`, `allophone`, `numerals`,
  `dialect` and `error` are now at 100%, and `error.cyr` went from no referenced fn at
  all to covered. 222 new assertions, 17 → 19 test files (21 with the two above).
  - **swadesh** — `tests/swadesh.tcyr` (+100): all ten list builders called directly
    instead of only through the `swadesh_by_code` dispatch table, each checked for
    code, 25 entries and the first/last entry's word, IPA, gloss and Swadesh index;
    `swadesh_entry` / `_word` / `_ipa` / `_gloss` pinned against the shared gloss set.
  - **syllable** — `tests/syllable.tcyr` (+42): `SyllableTemplate` accessors read
    directly and cross-checked against the delegating `phonotactics_*` ones;
    `phonotactics_language_code` / `_syllable` / `_constraints`; `constraint_kind` /
    `_position` / `_sequences` / `_description` over the English, Japanese and
    Sanskrit profiles; and an open-syllable (`max_coda 0`) template for the coda
    thresholds no pre-built profile reaches.
  - **lexicon** — new `tests/lexicon.tcyr` (24): `LexEntry` field accessors, the
    `(0-1)` None sentinel for `frequency_rank` / `swadesh_index`, `lexicon_entries`
    insertion order, and `lexicon_find_gloss` vs `lexicon_find` on a non-English
    lexicon where native word and English gloss cannot agree.
  - **numerals** — `tests/numerals.tcyr` (+19): `numerals_script_code` / `_name` /
    `_mappings` for all five systems, pinning each Unicode script tag and table size.
  - **allophone** — `tests/allophone.tcyr` (+11): `allophone_rule_phoneme` /
    `_allophone` / `_environment` / `_obligatory` — the environment and obligatoriness
    that `allophone_realize` collapses away.
  - **dialect** — `tests/dialect.tcyr` (+11): `dialect_code` / `_name` / `_added` /
    `_removed`, and the RP intervocalic-/t/ override read through the rule accessors.
  - **error** — new `tests/error.tcyr` (15): `varna_error_str` for every `VarnaError`
    code plus the out-of-range fallback, so an unknown sentinel still renders.

### Performance

Measured by interleaving old and new builds in one session (`cyrius bench`, min
ns) — a straight before/after against an earlier baseline showed a phantom
15-25% regression across untouched rows that was pure machine drift.

- **registry_all_codes_iter** — 42,499 → 3,310 ns (**-92%**, 12.8x).
  `registry_info` was a linear `streq` scan over 51 entries, so iterating the
  registry cost ~1,300 string compares; it is now a lazily built `map_get`.
- **transliterate_greek_word** — 39,636 → 19,351 ns (**-51%**). `translit_apply`
  allocated a `lens` vec plus up to four NUL-terminated candidate buffers *per
  input grapheme*; matching is now done in place against the table literals.
- **numeral_string_value_word** — 1,591 → 969 ns (**-39%**); the per-character
  allocation is gone the same way.
- **numeral_value_of_char** — 502 → 310 ns (**-38%**) and
  **transliterate_devanagari_char** — 916 → 575 ns (**-37%**): one `strlen` of the
  needle per call instead of one per table entry.
- Every other row is flat within its run-to-run noise. `script_contains_codepoint`
  reads 16 → 14 ns but that is inside its own ±14% spread and nothing in
  `script.cyr` changed — not claimed as a win.


## [2.1.1] - 2026-08-27

Toolchain-maintenance release: Cyrius upgrade and dependency refresh. No changes to
the public API or linguistic data — the default build, the full
`-D LOGGING -D MCP -D DAIMON -D HOOSH` build, and every test stay green against the
newer stdlib.

### Changed

- **Toolchain** — Cyrius pin bumped `6.4.69` → `6.5.35` (`cyrius.cyml [package].cyrius`);
  CI and the release workflow derive their install version from the pin, so the whole
  pipeline moves with it.
- **Dependencies** — vendored Cyrius stdlib re-resolved against the 6.5.35 snapshot.
  The closure is the same 29 modules as at 2.1.0, but 20 of them changed content —
  most substantially `bayan` (TOML/JSON parser, ~10.8k changed lines), `syscalls_x86_64_agnos`,
  `alloc`, `bench`, `io`, `vec`, and the per-arch `syscalls_*` set. Upstream added
  `async_macos`/`thread_macos` to the snapshot; neither is in varna's closure.
- **cyrius.lock** — regenerated at 6.5.35; `cyrius deps --verify` is clean (29 verified,
  0 failed). Resolved with `cyrius deps`, whose include-graph walk is the authority:
  6.5.35's new `cyrius lib sync` matches on declared `[deps].stdlib` names instead and
  produces a *different* 29-module set — it adds the unreferenced `hashmap_fast.cyr`
  and drops `atomic.cyr`, which `lib/alloc.cyr` includes transitively and the build
  needs. Locking that set would break a fresh `cyrius deps --verify`.
- **bote** — the commented-out `[deps.bote]` recipe in `cyrius.cyml` (inert; `-D MCP`
  hand-builds its JSON today) refreshed from the stale `tag = "2.7.6"` to `3.3.7`,
  which pins the same Cyrius 6.5.35 toolchain. Its companion notes were corrected
  against the real `dist/bote-core.deps` sidecar: the profile is 12 modules (not 9),
  and the only stdlib leaf varna does not already declare is `chrono` — not the
  `ct`/`keccak`/`random`/`thread`/`thread_local`/`sigil`/`freelist` set the old
  comment listed.
- **daimon** — agent-registration version string bumped to `2.1.1` (`src/daimon.cyr`).
- **dist** — bundles regenerated at `2.1.1`. `cyrius distlib` now also emits a `.deps`
  stdlib-leaf sidecar per profile, and `cyrius distlib --all` writes the `core` profile
  bundle, so three files join the tracked `dist/`: `varna.deps` (16 leaves),
  `varna-core.cyr`, and `varna-core.deps` (4 leaves — `string`, `alloc`, `vec`,
  `hashmap`). `[lib]` and `[lib.core]` declare identical module lists, so the two
  bundle bodies are byte-identical; the sidecar is what distinguishes them.
- **scripts/check.sh** — the gate now runs `cyrius deps --verify` after `cyrius deps`,
  so a lock that does not match the snapshot it claims to pin fails locally and in CI
  instead of shipping (see *Fixed* below).
- **scripts/bench-history.sh** — now records the toolchain version and the harness's
  measured timer floor in `BENCHMARKS.md`, plus a note marking the pre-/post-6.5.19
  measurement discontinuity. On a parse failure it echoes the captured output instead
  of failing silently.

### Fixed

- **cyrius.lock** — the lock shipped at 2.1.0 recorded a wrong hash for
  `lib/syscalls_x86_64_agnos.cyr` (`c5e63ab…` against the 6.4.69 snapshot's actual
  `e53e129…`), so `cyrius deps --verify` reported `28 verified, 1 failed` on a clean
  checkout. CI never caught it because `scripts/check.sh` ran plain `cyrius deps`,
  not `--verify`. Regenerating at 6.5.35 clears it, and the gate now runs `--verify`
  so a stale lock cannot ship again.
- **Build warnings** — the gate is now warning-free. 6.4.69's vendored `lib/bayan.cyr`
  emitted three `assigning non-pointer to typed pointer` warnings on *every* compile
  (`_toml_parse_str`/`_toml_parse_multiline_q` were declared `: i64` but assigned into
  a `Str`); 6.5.35 declares them `: Str`. The `toolchain drift` warning is gone too,
  now that the pin matches the installed `cycc`.

### Performance

- **Stdlib refresh is performance-neutral.** Holding the benchmark harness constant at
  6.5.35 and swapping only the other 28 vendored modules, every benchmark lands within
  run-to-run noise (min ns, 6.4.69 → 6.5.35 stdlib): `english_phoneme_inventory`
  1272 → 1341, `registry_all_codes_iter` 42306 → 42820, `transliterate_greek_word`
  39686 → 40632, `dialect_apply_overlay` 2906 → 3215. Binary size is unchanged at
  597,976 bytes for the default build.
- **The recorded numbers moved, but the instrument moved — not the code.** Cyrius
  6.5.19 rewrote `lib/bench.cyr`: `bench_run` used to wrap a clock pair around every
  iteration, flooring all 18 varna benchmarks at roughly two clock reads (the
  ~419ns/489ns/907ns plateaus visible throughout the old history). It now sizes its own
  batches and subtracts a *measured* per-host clock cost (1.409us on the recording
  host). Sub-microsecond rows finally resolve — `script_contains_codepoint` reads 14ns
  and `allophone_realize` 34ns where both previously bottomed out at 419ns — and the
  heavier rows shift because they are no longer inflated by an unsubtracted floor.
  Rows in `bench-history.csv` on either side of 2026-08-27 are **not** comparable;
  the floor is a property of the host clocksource and moves between reboots. See
  `BENCHMARKS.md`.

## [2.1.0] - 2026-07-21

Toolchain-maintenance release: Cyrius upgrade and dependency refresh. No changes to
the public API or linguistic data — the default build, the full
`-D LOGGING -D MCP -D DAIMON -D HOOSH` build, and every test stay green against the
newer stdlib.

### Changed

- **Toolchain** — Cyrius pin bumped `6.2.12` → `6.4.69` (`cyrius.cyml [package].cyrius`);
  CI derives its install version from the pin, so the whole pipeline moves with it.
- **Dependencies** — vendored Cyrius stdlib re-resolved against the 6.4.69 snapshot. Upstream
  dropped `agnosys`; `async_win`/`protobuf`/`yantra` are new (none used by varna).
- **cyrius.lock** — regenerated at 6.4.69. Now pins the exact 29-module dependency closure a
  fresh `cyrius deps` resolves, so `cyrius deps --verify` is clean (29 verified, 0 failed).
  Removed 8 stale entries that were never actually resolved or linked by any build config
  (`agnosys`, `bote-core`, `libro`, `majra`, `log`, `sakshi`, `sigil`, `patra`) and added the
  benchmark-harness dependency `bench`.
- **daimon** — agent-registration version string bumped to `2.1.0` (`src/daimon.cyr`).
- **dist** — `dist/varna.cyr` bundle regenerated (header `# Version: 2.1.0`).

### Fixed

- **scripts/bench-history.sh** — `to_ns()` now parses the fractional-unit benchmark
  output Cyrius ≥6.3 emits (`1.396us`); the old converter only stripped leading integer
  digits, so decimal timings were written to `bench-history.csv` unscaled (`1.396` instead
  of `1396`). Results are rounded to integer nanoseconds, matching the existing history.

### Performance

- No regressions from the toolchain move (min ns, 6.2.12 → 6.4.69): `english_phoneme_inventory`
  2000 → 1396, `transliterate_greek_word` 38000 → 36527, `registry_all_codes_iter` 37000 → 36666;
  all other benchmarks within noise. See `bench-history.csv` / `BENCHMARKS.md`.

## [2.0.0] - 2026-06-16

Ported from a Rust crate to the [Cyrius](https://github.com/MacCracken/cyrius)
systems language. See [ADR 0001](docs/adr/0001-port-from-rust-to-cyrius.md) for the
full rationale. The `content`/data model and the public surface are preserved at
parity; the implementation language, build system, and dependency stack changed.

### Breaking

- **Language** — varna is now a Cyrius library (`dist/varna.cyr`), not a Rust crate.
  The v1.x Rust source is frozen in `rust-old/`. Consumers depend on varna via a
  `[deps.varna]` block in `cyrius.cyml` (`modules = ["dist/varna.cyr"]`), not Cargo.
- **API shape** — the Rust module-path API (`varna::phoneme::english()`) is re-exposed
  as snake-case free functions (`phoneme_english()`); `Option`/`unwrap` access becomes
  sentinel-return checks; `Result` errors use native tagged `enum`s.
- **Feature flags → build defines** — the Cargo `std`/`logging`/`mcp`/`daimon`/`hoosh`/`full`
  features are replaced by `-D LOGGING`/`-D MCP`/`-D DAIMON`/`-D HOOSH` passed to
  `cyrius build`. There is no `std`/`no_std` split.
- **Renames for project-name consistency** — `LIPI_LOG` → `VARNA_LOG`; `LipiError` →
  `VarnaError`; MCP tools `lipi_*` → `varna_*` (`varna_phonemes`, `varna_script`,
  `varna_grammar`, `varna_translate_ipa`, `varna_compare`); `ResponseSource::LipiData`
  → `VarnaData`.

### Changed

- **Dependency stack** — every external Rust dependency now resolves to the Cyrius
  stdlib or a language builtin:
  - `serde` + `serde_json` → `bayan` (`#derive(Serialize)` + `json_parse`/`json_build`)
  - `thiserror` → native `enum` + `Result<T,E>` (`lib/result.cyr`, `lib/tagged.cyr`)
  - `tracing` + `tracing-subscriber` → `lib/log.cyr` + `lib/sakshi.cyr` (env-filter wired by hand)
  - `criterion` → `cyrius bench` + `lib/bench.cyr`
  - Optional `bote` (MCP) → consumed as the `dist/bote-core.cyr` git dep under `-D MCP`
- **String model** — `Cow<'static, str>` fields become `'static` `str` literals (Cyrius
  has no `Cow`); the builder still constructs inventories at runtime.
- **Enums** — `#[non_exhaustive]` enums become Cyrius tagged enums (additive variants).
- **Build + test layout** — `Cargo.toml`/`Cargo.lock` → `cyrius.cyml`/`cyrius.lock`;
  `benches/benchmarks.rs` (criterion) → `benches/*.bcyr`; `tests/integration.rs` →
  `tests/tcyr/*.tcyr`; `make check` (cargo fmt/clippy/test/audit) → `cyrius audit`.

### Removed

- The Rust toolchain config (`rust-toolchain.toml`), `deny.toml`, and the Cargo
  feature system. `cargo doc` browsable API reference (varna's surface is a folded
  `.cyr` bundle, navigable in `src/`).
- The `std`/`no_std`/`alloc` feature distinction — Cyrius emits a single artifact.

### Performance

- Baselined 18 benchmarks via `cyrius bench tests/varna.bcyr` (`bench-history.csv` /
  `BENCHMARKS.md`). Representative per-op `min` figures: phoneme lookup 419ns,
  `script_by_code` 838ns, grammar lookup 419ns, English inventory build 2µs, registry
  iteration over 51 languages 40µs.

### Note

The source-level port is **complete**: all 19 modules (15 core + 4 `-D` surfaces) are
reimplemented as `src/*.cyr`, verified by **526 parity assertions** (`cyrius tests`)
against the frozen `rust-old/` oracle plus a 7-agent parity audit (0 unported public
functions). Documented divergences from the oracle: sentinel returns for `Option`/`Result`,
integer-tagged enums, hand-built JSON (`mcp` uses a lightweight `ToolDef`, no `bote`/`bayan`),
`hoosh` confidence as per-mille, and the name-colliding enum variants `WordOrder.Free` /
`VarietyKind.Historical` / `PhonemeClass` omitted. `daimon`/`hoosh` remain Cyrius binaries
with no consumable library bundle, so those define-gated surfaces only toggle JSON output.

## [1.0.0] - 2026-03-31

### Added

- **phoneme::inventories** — 24 additional language inventories reaching 51 total:
  - European: Italian (23C+7V), Dutch (19C+13V), Polish (29C+6V), Czech (25C+10V), Hungarian (25C+14V), Romanian (21C+7V), Icelandic (22C+8V)
  - South Asian: Bengali (30C+7V), Tamil (19C+10V), Urdu (37C+10V)
  - African: Amharic (27C+7V), Hausa (25C+10V), Somali (22C+10V), Wolof (25C+8V)
  - Southeast Asian: Indonesian (18C+6V), Burmese (30C+7V), Khmer (24C+18V), Lao (20C+9V)
  - Middle East/Caucasus: Persian (23C+6V), Hebrew (23C+5V), Georgian (28C+5V)
  - Americas: Quechua (24C+3V), Guarani (16C+12V)
  - Central Asian: Mongolian (21C+14V)
- **grammar** — English grammar profile added (`grammar::by_code("en")`)
- **docs** — Usage guide (docs/guides/usage.md) with 15 sections and code examples
- **docs** — Architecture overview fully updated for all modules and 51 languages

### Changed

- **registry** — 51 languages registered (up from 27)
- v1.0 criteria: test coverage measured at 98.53%, all documentation complete

## [0.6.0] - 2026-03-31

### Added

- **mcp** — MCP tool definitions (feature-gated `mcp`): `lipi_phonemes`, `lipi_script`, `lipi_grammar`, `lipi_translate_ipa`, `lipi_compare`. Tool registry with `tool_definitions()`, `invoke()` dispatcher, JSON-serializable `ToolResult`. Language comparison computes shared/unique phonemes and grammar differences
- **daimon** — Agent registration for AGNOS framework (feature-gated `daimon`): `AgentRegistration` with 6 capabilities, dynamic language/script coverage from registry. Version auto-synced from Cargo.toml
- **hoosh** — LLM query interface (feature-gated `hoosh`): `LanguageQuery` enum (5 query types), `QueryResponse`, `ResponseSource`. `answer_from_data()` handles phoneme/comparison queries without LLM, returns `None` for queries requiring inference
- Feature gates: `mcp`, `daimon`, `hoosh` (all included in `full`)

## [0.5.0] - 2026-03-31

### Added

- **phoneme::inventories** — 9 core language inventories: Mandarin (21C+7V), Hindi (34C+10V), Japanese (20C+5V), Spanish (23C+5V), French (21C+16V), German (23C+16V), Russian (36C+6V), Korean (19C+7V), Portuguese (23C+14V)
- **grammar** — Pre-built grammar profiles for 10 core languages with `by_code()` lookup and `all_codes()`. Covers morphology, word order, case count, gender, dual number, classifiers
- **lexicon::swadesh** — Swadesh-25 starter word lists for 10 core languages (250 entries total) with IPA transcription, part of speech, and Swadesh index. `by_code()` lookup
- **registry** — 27 languages registered (up from 18)

## [0.4.1] - 2026-03-31

### Added

- **script** — `[S]` Cuneiform script metadata (Xsux) with Sumerian/Akkadian language tags and Unicode ranges
- **script** — `[S]` Egyptian hieroglyphic script metadata (Egyp) with Unicode ranges
- **script** — `ScriptStatus` enum (Living, Limited, Historical) and `attestation` period field on all scripts
- **script::numerals** — `[S]` Babylonian sexagesimal numeral system (base-60, cuneiform digits)
- **script::numerals** — `[S]` Egyptian hieroglyphic numeral system (additive decimal, powers of 10)
- **script::numerals** — `[S]` Chinese rod numeral system (positional decimal, vertical forms)
- **phoneme::inventories** — 4 classical/liturgical language inventories:
  - **Latin** (la) — 18C + 10V, labialized velars
  - **Classical Arabic** (ar) — 28C + 6V, pharyngeals, emphatics, uvular
  - **Koine Greek** (grc) — 17C + 5V, pitch accent
  - **Literary Chinese** (lzh) — 27C + 12V, 4-tone Middle Chinese reconstruction
- **registry** — 18 languages registered (up from 14), 10 scripts (up from 8)

## [0.4.0] - 2026-03-31

### Added

- **dialect** — Language variety support: `LanguageVariety`, `VarietyKind` (Regional, NationalStandard, Historical, Sociolect, Creole). Phoneme add/remove overlays and allophone overrides. British English (RP) and Egyptian Arabic pre-built
- **lexicon::cognate** — Cognate detection types: `CognateSet` with proto-form and cross-language entries, `Etymology` with `BorrowingType` (Loanword, Calque, SemanticLoan, Inherited). PIE "water" cognate set pre-built
- **phoneme::inventories** — 11 new language inventories across 6 language families:
  - `[S]` **Yucatec Maya** (yua) — 21C + 10V, ejective consonants for Mayan calendar validation
  - **Swahili** (sw) — 26C + 5V, **Yoruba** (yo) — 18C + 7V (3-tone), **Zulu** (zu) — 42C + 5V (clicks)
  - **Thai** (th) — 21C + 9V (5-tone), **Vietnamese** (vi) — 22C + 11V (6-tone), **Tagalog** (tl) — 18C + 5V
  - **Turkish** (tr) — 20C + 8V (vowel harmony), **Finnish** (fi) — 17C + 16V (short+long)
  - **Hawaiian** (haw) — 8C + 10V (minimal inventory), **Nahuatl** (nah) — 16C + 8V (lateral affricate)
- **registry** — 14 languages registered (up from 3)

## [0.3.0] - 2026-03-31

### Added

- **phoneme::allophone** — Allophone rule system: `AllophoneRuleSet`, `AllophoneRule`, `Environment`, `PhonemeClass`. Context-dependent sound variation with `rules_for()` and `realize()` lookup. English (GA) rules included (aspiration, flapping, dark-l)
- **phoneme::syllable** — Syllable structure templates: `SyllableTemplate` with max onset/coda, `Phonotactics` with `PhonotacticConstraint`. English, Sanskrit, and Japanese profiles included
- **script::transliteration** — `[S]` Bidirectional transliteration tables: `TransliterationTable` with `transliterate()`, `transliterate_char()`, and `reverse_map()`. Devanagari↔IAST and Greek↔Beta Code tables included
- **script::numerals** — `[S]` Script-to-numeral mapping: `NumeralSystem` with `value_of()`, `char_for()`, and `string_value()`. Devanagari decimal digits and Greek isopsephy included
- `Hash` derive on `Phoneme`, `PhonemeKind`, `Morphology`, `WordOrder`, `Direction`
- `debug_assert` for duplicate IPA detection in `PhonemeInventoryBuilder::build()`

### Changed

- **script** — Kana script name corrected from "Katakana" to "Kana (Hiragana + Katakana)"
- **registry** — `all_codes()` returns `&'static [&'static str]` instead of allocating `Vec`
- **lib.rs** — Crate documentation updated to reflect five modules (added registry)
- **docs** — Architecture overview rewritten with registry, script/language tables

## [0.2.0] - 2026-03-31

### Added

- **phoneme** — `PhonemeInventoryBuilder` for ergonomic inventory construction with `consonant()`, `vowel()`, `stress()`, `tones()`, and `with_capacity()` methods
- **phoneme** — Sanskrit (Classical) phoneme inventory: 36 consonants + 15 vowels, organized by 5 vargas (consonant groups for Katapayadi encoding) `[S]`
- **phoneme** — Greek (Modern Standard) phoneme inventory: 20 consonants + 5 vowels `[S]`
- **script** — Pre-built metadata for 8 writing systems: Latin, Arabic, Devanagari, CJK, Cyrillic, Hangul, Kana, Greek `[S]`
- **script** — `by_code()` lookup by ISO 15924 code, `all_codes()` enumeration, `Script::contains_codepoint()` for Unicode range checking
- **registry** — Language registry with ISO 639 lookup: `info()`, `phonemes()`, `primary_script()`, `all_codes()`
- **phoneme** — `LabialVelar` place of articulation variant
- **phoneme** — `Phoneme::consonant()` and `Phoneme::vowel()` constructors for external use
- **phoneme** — `PartialEq`/`Eq` derives on `Phoneme`, `PhonemeKind`, `PhonemeInventory`
- **script** — `PartialEq`/`Eq` derives on `Script`
- **grammar** — `PartialEq`/`Eq` derives on `GrammarProfile`
- **lexicon** — `PartialEq`/`Eq` derives on `LexEntry`, `Lexicon`
- 6 criterion benchmarks (3 inventories, phoneme lookup, registry lookup, script lookup)

### Changed

- **phoneme** — All string fields migrated to `Cow<'static, str>` for zero-alloc static inventories
- **phoneme** — English `/w/` reclassified from `Bilabial` to `LabialVelar` (linguistically accurate)
- **phoneme** — `english()` refactored to use `PhonemeInventoryBuilder`
- **phoneme** — `#[non_exhaustive]` added to `PhonemeKind::Consonant` and `PhonemeKind::Vowel` variants
- **script** — All string fields migrated to `Cow<'static, str>`
- **grammar** — `language_code` migrated to `Cow<'static, str>`
- **lexicon** — All string fields migrated to `Cow<'static, str>`
- Tracing instrumentation added to all public lookup methods

### Performance

- `english_phoneme_inventory`: 146ns (builder with pre-alloc)
- `sanskrit_phoneme_inventory`: 197ns
- `greek_phoneme_inventory`: 90ns
- `phoneme_lookup_ipa`: 14ns
- `registry_phonemes_lookup`: 200ns
- `script_by_code_lookup`: 19ns

## [0.1.0] - 2026-03-30

### Added

- **phoneme** — IPA phoneme inventories with articulatory features (manner, place, voicing, height, backness, rounding), stress patterns, tone systems. English (General American) inventory included
- **script** — Writing system metadata: alphabet, syllabary, logographic, abjad, abugida, mixed. Unicode ranges, directionality (LTR, RTL, TTB, bidi)
- **grammar** — Morphological typology (isolating, agglutinative, fusional, polysynthetic), word order (SVO/SOV/VSO/VOS/OVS/OSV/Free), case systems, gender, dual number, classifiers
- **lexicon** — Lexical entries with IPA transcription, part of speech, frequency ranking, Swadesh list indexing. Lookup, Swadesh extraction, frequency ranking
- **error** — `LipiError` with variants for unknown language/script, missing phonemes, invalid IPA, word-not-found
- **logging** — Optional structured logging via `LIPI_LOG` env var (feature-gated)
- Initial criterion benchmarks for phoneme inventory construction and lookup
