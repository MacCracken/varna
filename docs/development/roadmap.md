# Development Roadmap

> **Status**: v2.1 (Cyrius) | **Current**: 2.1.2
>
> Items marked `[S]` also unblock **sankhya** (ancient mathematical systems).
>
> The 0.x–1.0 history below records the **Rust crate** (frozen in `rust-old/`).
> Mechanics named there (`Cow`, `#[non_exhaustive]`, `criterion`, `serde`) are
> historical; their Cyrius equivalents are in [ADR 0001](../adr/0001-port-from-rust-to-cyrius.md).
> Backlog signatures are illustrative — they land as Cyrius forms (tagged enums,
> sentinel returns, `str` literals; no `Cow`/`Option` types).

## Completed

### 2.1.2 — Hardening Sweep (2026-08-27)

P(-1) scaffold sweep: audit, repair, optimise.

- [x] **Heap overflow, `-D MCP`** — `_tool_err2` wrote an unbounded caller-supplied
      parameter into a fixed `alloc(256)`; reachable from all five tools and the
      unknown-tool path. All appends now carry the buffer capacity
- [x] **Heap overflow, `-D HOOSH`** — same defect via `hoosh_answer_from_data`'s
      unmatched `ipa` string
- [x] **Out-of-bounds read** — `_utf8_len` trusts the lead byte, so a truncated
      codepoint at the tail let `translit_apply` and `numerals_string_value` read past
      the input. New `_utf8_step` clamps every stride to the bytes present
- [x] **Output overflow** — `translit_apply` sized its buffer at `inlen * 4`, sound
      only for tables that contract; now sized from the table's longest target and
      bounds-checked regardless
- [x] `registry_info` indexed (linear `streq` scan → `map_get`); `registry_all_codes`
      cached instead of rebuilt per call — **the returned vec is now shared, read-only**
- [x] Per-grapheme and per-character allocations removed from the transliteration and
      numeral hot paths (the bump allocator never frees)
- [x] `registry_phonemes` tied to `_registry_build` by test; `phoneme_builder_with_capacity`'s
      ignored `cap` argument documented
- [x] 54 new assertions across `tests/hardening.tcyr` + `tests/hardening_surfaces.tcyr`,
      each verified to fail against the pre-fix code
- [x] `registry_all_codes_iter` -92%, `transliterate_greek_word` -51%,
      `numeral_string_value_word` -39%, `numeral_value_of_char` -38%,
      `transliterate_devanagari_char` -37%; all other rows flat (interleaved A/B)

### 2.1.1 — Toolchain Maintenance (2026-08-27)

- [x] Cyrius toolchain pin bumped `6.4.69` → `6.5.35` (`cyrius.cyml [package].cyrius`)
- [x] Vendored Cyrius stdlib re-resolved against the 6.5.35 snapshot — same 29-module
      closure, 20 modules changed content (`bayan` most heavily, ~10.8k lines)
- [x] `cyrius.lock` regenerated at 6.5.35 (`cyrius deps --verify` clean, 29/0) — and the
      2.1.0 lock's wrong hash for `lib/syscalls_x86_64_agnos.cyr` fixed
- [x] `cyrius deps` (include-graph walk) confirmed as the lock authority over 6.5.35's new
      `cyrius lib sync`, which name-matches `[deps].stdlib` and resolves a closure missing
      `atomic.cyr` (a transitive include of `lib/alloc.cyr`)
- [x] Commented `[deps.bote]` recipe refreshed `2.7.6` → `3.3.7` and its companion notes
      corrected against `dist/bote-core.deps` (12-module profile; `chrono` is the one
      undeclared stdlib leaf)
- [x] `dist/` gains the `cyrius distlib` sidecars + core profile bundle (`varna.deps`,
      `varna-core.cyr`, `varna-core.deps`)
- [x] Build gate now warning-free (6.5.35's `bayan` retyped the TOML string parsers that
      emitted three pointer warnings per compile at 6.4.69)
- [x] `scripts/bench-history.sh` records the toolchain version + measured timer floor, and
      marks the pre-/post-6.5.19 benchmark-harness discontinuity in `BENCHMARKS.md`
- [x] Stdlib refresh proved performance-neutral (harness held constant; all rows in noise)
- [x] No public-API or linguistic-data changes; every build config + all tests stay green

### 2.1.0 — Toolchain Maintenance (2026-07-21)

- [x] Cyrius toolchain pin bumped `6.2.12` → `6.4.69` (`cyrius.cyml [package].cyrius`)
- [x] Vendored Cyrius stdlib re-resolved against the 6.4.69 snapshot (`cyrius update`)
- [x] `cyrius.lock` regenerated at 6.4.69 — pins the exact 29-module dependency closure
      (`cyrius deps --verify` clean); dropped 8 vestigial entries never resolved or built
      (`agnosys`/`bote-core`/`libro`/`majra`/`log`/`sakshi`/`sigil`/`patra`), added `bench`
- [x] No public-API or linguistic-data changes; every build config + all tests stay green

### 2.0.0 — Cyrius Port (2026-06-16)

- [x] Ported from a Rust crate to the Cyrius systems language ([ADR 0001](../adr/0001-port-from-rust-to-cyrius.md))
- [x] `cyrius.cyml` manifest: `[package]` + `[build]` + `[lib]`/`[lib.core]` + `[deps]`
- [x] Dependency mapping to Cyrius stdlib (serde/serde_json→`bayan`, thiserror→`result`/`tagged`, tracing→`log`/`sakshi`, criterion→`cyrius bench`); `bote-core` git dep under `-D MCP`
- [x] Feature flags → `-D` build defines (`LOGGING`/`MCP`/`DAIMON`/`HOOSH`)
- [x] Legacy `lipi`/`LIPI` naming retired → `varna`/`VARNA`
- [x] Documentation ported (README, CLAUDE.md, architecture, CONTRIBUTING, SECURITY, this roadmap)
- [x] Source-level reimplementation of every domain as `src/*.cyr` — 19 modules (15 core + 4 `-D` surfaces), 523 parity assertions green against the `rust-old/` oracle

### 0.1.0 — Scaffold (2026-03-30)

- [x] Core type system: Phoneme, Script, GrammarProfile, Lexicon, LexEntry
- [x] Articulatory features: Manner, Place, Height, Backness, voicing, rounding
- [x] Writing system classification: Alphabet, Abugida, Abjad, Syllabary, Logographic, Mixed
- [x] Grammar typology: Isolating, Agglutinative, Fusional, Polysynthetic
- [x] Word order variants: SVO, SOV, VSO, VOS, OVS, OSV, Free
- [x] Lexicon with Swadesh indexing and frequency ranking
- [x] English (General American) phoneme inventory
- [x] Error types with thiserror
- [x] Optional structured logging
- [x] Initial criterion benchmarks

### 0.1.0 — Scaffold Hardening (2026-03-31)

- [x] Cow<'static, str> migration for zero-alloc static inventories
- [x] PartialEq/Eq derives on all public types
- [x] #[non_exhaustive] on PhonemeKind variants with Phoneme::consonant/vowel constructors
- [x] LabialVelar place of articulation, /w/ reclassified
- [x] Tracing instrumentation on public methods
- [x] Expanded test coverage (29 tests)
- [x] Cargo.lock removed from tracking

### 0.2.0 — Sankhya Foundation & Script Registry (2026-03-31)

- [x] `[S]` Sanskrit phoneme inventory (36 consonants + 15 vowels, 5 vargas for Katapayadi)
- [x] `[S]` Greek phoneme inventory (20 consonants + 5 vowels)
- [x] Script metadata for: Latin, Arabic, Devanagari, CJK, Cyrillic, Hangul, Kana
- [x] `[S]` Script metadata for: Greek alphabet (Unicode range, directionality)
- [x] Builder pattern for PhonemeInventory construction (`PhonemeInventoryBuilder`)
- [x] Language registry with ISO 639 lookup (`registry` module)

### 0.3.0 — Allophone & Phonotactics (2026-03-31)

- [x] Allophone rules per language (`phoneme::allophone` — Environment, PhonemeClass, AllophoneRule, AllophoneRuleSet)
- [x] Phonotactic constraints (`phoneme::syllable` — PhonotacticConstraint, ConstraintKind, Phonotactics)
- [x] Syllable structure templates (SyllableTemplate — onset/nucleus/coda, English/Sanskrit/Japanese profiles)
- [x] `[S]` Romanization/transliteration tables (`script::transliteration` — Devanagari↔IAST, Greek↔Beta Code)
- [x] `[S]` Script-to-numeral mapping API (`script::numerals` — Devanagari digits, Greek isopsephy)

### 0.4.0 — Extended Coverage (2026-03-31)

- [x] `[S]` Yucatec Maya phoneme inventory (21C + 10V, ejectives for Mayan calendar)
- [x] 11 additional language inventories: Swahili, Yoruba, Zulu, Thai, Vietnamese, Tagalog, Turkish, Finnish, Hawaiian, Nahuatl (14 total languages)
- [x] Dialect/variety support (`dialect` module — LanguageVariety, VarietyKind, phoneme overlays)
- [x] Cognate detection types (`lexicon::cognate` — CognateSet, CognateEntry, proto-forms)
- [x] Loanword tracking and etymology (`Etymology`, `BorrowingType`)

### 0.4.1 — Classical & Ancient Scripts (2026-03-31)

- [x] `[S]` Cuneiform script metadata (Xsux) + Babylonian sexagesimal numeral system
- [x] `[S]` Egyptian hieroglyphic script metadata (Egyp) + additive decimal numeral system
- [x] `[S]` Chinese rod numeral system (positional decimal, vertical forms)
- [x] Classical/Liturgical language profiles: Latin, Classical Arabic, Koine Greek, Literary Chinese (Sanskrit already in 0.2.0)
- [x] Dead script classification: `ScriptStatus` (Living/Limited/Historical) + attestation periods

### 0.5.0 — Core Languages (2026-03-31)

- [x] Language inventories: Mandarin, Hindi, Japanese, Spanish, French, German, Russian, Korean, Portuguese (Arabic in 0.4.1)
- [x] Grammar profiles for all 10 core languages (`grammar::by_code()`)
- [x] Swadesh-25 starter lists for each language (`lexicon::swadesh::by_code()`, 250 entries)

### 0.6.0 — AI Integration (2026-03-31)

- [x] MCP tools: `lipi_phonemes`, `lipi_script`, `lipi_grammar`, `lipi_translate_ipa`, `lipi_compare` (feature-gated `mcp`)
- [x] Daimon agent registration: `AgentRegistration` with 6 capabilities (feature-gated `daimon`)
- [x] Hoosh LLM query interface: `LanguageQuery`, `answer_from_data()` for data-only resolution (feature-gated `hoosh`)

## Deferred from 2.1.2

### Cache the pre-built data constructors

Every `phoneme_*` / `script_*` / `grammar_*` / `swadesh_*` / `translit_*` /
`numerals_*` constructor rebuilds its whole structure on each call —
`phoneme_english()` allocates an inventory, a vec and 36 phoneme records every
time, and `registry_phonemes` calls it fresh on every lookup. With a bump
allocator that never frees, a consumer polling the registry leaks steadily, and
this is the single largest remaining cost in the inventory benchmarks (the
`*_phoneme_inventory` rows are almost entirely construction).

Deferred out of the 2.1.2 hardening sweep deliberately: caching turns each
constructor into a shared singleton, so a consumer mutating a returned inventory
through the `phoneme_builder_*` functions would corrupt every other caller's
copy. That is a public-API semantic change and wants its own release with a
migration note — `registry_all_codes` took exactly this change in 2.1.2 and is
now documented read-only. Options: cache and document read-only; or cache plus an
explicit `phoneme_clone` for callers that need a mutable copy.

## Post-1.0 Roadmap — "World's Leading Authority"

> Gaps identified by comparing varna against PHOIBLE, WALS, Glottolog, Unicode CLDR,
> and the IPA specification. Prioritized by impact on credibility and utility.

### 1.1.0 — Phonological Depth (P1)

- [ ] **Distinctive feature system**: Add `DistinctiveFeatures` bundle with 20+ binary features per phoneme (sonorant, continuant, strident, anterior, distributed, ATR/RTR, spread/constricted glottis, syllabic, etc.) — PHOIBLE parity
- [ ] **Manner expansion**: Add `Click`, `Implosive`, `Ejective` to `Manner` enum — reclassify Zulu clicks, Georgian ejectives, Hausa implosives
- [ ] **Consonant secondary features**: `aspirated`, `labialized`, `palatalized`, `prenasalized`, `long` fields on the `PhonemeKind.Consonant` variant
- [ ] **Vowel features**: `long`, `nasalized`, `atr` (Advanced Tongue Root) fields on the `PhonemeKind.Vowel` variant
- [ ] **Tone as structured data**: Replace string tone labels with `Tone` structs (contour, register, features)

### 1.2.0 — Typological Depth (P2)

- [ ] **Grammar expansion** toward WALS parity: alignment type (nom-acc/erg-abs/active-stative), adposition order, tense/aspect system, evidentiality, negation strategy, adjective order, relative clause order, article type
- [ ] **Language classification**: Add `family`, `subfamily`, `genus` to `LanguageInfo` (Indo-European > Germanic > West Germanic)
- [ ] **Missing script entries**: Hebrew, Thai, Tamil, Georgian, Ethiopic, Myanmar, Khmer, Lao, Bengali (9 scripts for already-registered languages)
- [ ] **Glottocode support**: Add a nullable `glottocode` field (str) alongside ISO 639 codes
- [ ] **Endangerment status**: `EndangermentLevel` enum (Safe/Vulnerable/Threatened/Shifting/Moribund/NearlyExtinct/Extinct)
- [ ] **Geographic metadata**: Latitude/longitude per language, macro-area classification

### 1.3.0 — Gematria & Numeric Letter Values (P2)

Extend `script::numerals` into a full character→number mapping system across scripts. Foundation for classical cipher work and sankhya gematria computation.

- [ ] **Hebrew gematria values**: Standard (א=1..ת=400), ordinal (א=1..ת=22), reduced (digital root)
- [ ] **Arabic abjad numerals**: Standard abjad order (أ=1..غ=1000)
- [ ] **Latin/English ordinal values**: a=1..z=26 (simple gematria, used by classical ciphers)
- [ ] **Cyrillic numeric values**: Church Slavonic letter-number system
- [ ] **`NumericSystem` enum**: Standard, Ordinal, Reduced, Additive — per-script system classification
- [ ] **`char_value(script, system, ch)`**: Unified lookup API across all scripts (returns the value, or a sentinel when unmapped)
- [ ] **`script_alphabet_values(script, system)`**: Full (char, value) mapping table per script
- [ ] **Cipher foundation**: Character↔number round-trip enables Caesar, Vigenère, substitution cipher implementations downstream (a crypto Cyrius project or sankhya)

### 1.4.0 — Coverage Scale (P3)

- [ ] **Data-driven inventories**: Load from PHOIBLE CSV/JSON for 2000+ languages (feature-gated)
- [ ] **Expanded allophone rules**: Mandarin, Spanish, Japanese, Russian, Arabic (currently English only)
- [ ] **Expanded phonotactic profiles**: All core languages (currently 3)
- [ ] **Transliteration tables**: Cyrillic-Latin, Arabic-Latin, Hebrew-Latin, Pinyin (currently 2)
- [ ] **Source provenance**: Track bibliography/reference for each inventory
- [ ] **Multiple inventories per language**: Competing analyses like PHOIBLE

### 1.5.0+ — Differentiators (P4)

- [ ] PHOIBLE-compatible export format
- [ ] WALS feature code mapping
- [ ] Typological cross-cutting queries ("all SOV languages with ejectives")
- [ ] ISO 639 validation (static lookup table)
- [ ] Prosody patterns (intonation contours, rhythm class: stress/syllable/mora-timed)
- [ ] Morphological analyzer (stemming, lemmatization per language)
- [ ] Historical phonology (sound change rules, Proto-IE reconstructions)
- [ ] Sign language phonology (handshape, location, movement features)
- [ ] ScriptType.Featural for Hangul reclassification

## v1.0 Criteria

- [x] 50+ language inventories with verified phoneme data (51 languages)
- [x] All modules have 80%+ test coverage (98.53% measured on the Rust crate; re-measured under `cyrius coverage` post-port)
- [x] `cyrius bench` benchmarks with 3-point trend history (`bench-history.csv`)
- [x] Full `bayan`-JSON roundtrip tests for all public types
- [ ] shabda + shabdakosh consuming varna for multilingual G2P (external Cyrius project work)
- [x] `[S]` sankhya consuming varna for script-aware numeral display and transliteration
- [x] Documentation: architecture overview, usage guide, API docs
- [x] English grammar profile added; 11 grammar profiles total
