# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
