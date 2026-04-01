# Development Roadmap

> **Status**: v1.0 | **Current**: 1.0.0
>
> Items marked `[S]` also unblock **sankhya** (ancient mathematical systems).

## Completed

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

## Post-1.0 Roadmap — "World's Leading Authority"

> Gaps identified by comparing varna against PHOIBLE, WALS, Glottolog, Unicode CLDR,
> and the IPA specification. Prioritized by impact on credibility and utility.

### 1.1.0 — Phonological Depth (P1)

- [ ] **Distinctive feature system**: Add `DistinctiveFeatures` bundle with 20+ binary features per phoneme (sonorant, continuant, strident, anterior, distributed, ATR/RTR, spread/constricted glottis, syllabic, etc.) — PHOIBLE parity
- [ ] **Manner expansion**: Add `Click`, `Implosive`, `Ejective` to `Manner` enum — reclassify Zulu clicks, Georgian ejectives, Hausa implosives
- [ ] **Consonant secondary features**: `aspirated`, `labialized`, `palatalized`, `prenasalized`, `long` fields on `PhonemeKind::Consonant`
- [ ] **Vowel features**: `long`, `nasalized`, `atr` (Advanced Tongue Root) fields on `PhonemeKind::Vowel`
- [ ] **Tone as structured data**: Replace `Vec<Cow<str>>` tone labels with `Tone` structs (contour, register, features)

### 1.2.0 — Typological Depth (P2)

- [ ] **Grammar expansion** toward WALS parity: alignment type (nom-acc/erg-abs/active-stative), adposition order, tense/aspect system, evidentiality, negation strategy, adjective order, relative clause order, article type
- [ ] **Language classification**: Add `family`, `subfamily`, `genus` to `LanguageInfo` (Indo-European > Germanic > West Germanic)
- [ ] **Missing script entries**: Hebrew, Thai, Tamil, Georgian, Ethiopic, Myanmar, Khmer, Lao, Bengali (9 scripts for already-registered languages)
- [ ] **Glottocode support**: Add `glottocode: Option<Cow<str>>` alongside ISO 639 codes
- [ ] **Endangerment status**: `EndangermentLevel` enum (Safe/Vulnerable/Threatened/Shifting/Moribund/NearlyExtinct/Extinct)
- [ ] **Geographic metadata**: Latitude/longitude per language, macro-area classification

### 1.3.0 — Gematria & Numeric Letter Values (P2)

Extend `script::numerals` into a full character→number mapping system across scripts. Foundation for classical cipher work and sankhya gematria computation.

- [ ] **Hebrew gematria values**: Standard (א=1..ת=400), ordinal (א=1..ת=22), reduced (digital root)
- [ ] **Arabic abjad numerals**: Standard abjad order (أ=1..غ=1000)
- [ ] **Latin/English ordinal values**: a=1..z=26 (simple gematria, used by classical ciphers)
- [ ] **Cyrillic numeric values**: Church Slavonic letter-number system
- [ ] **`NumericSystem` enum**: Standard, Ordinal, Reduced, Additive — per-script system classification
- [ ] **`char_value(script, system, char) -> Option<u32>`**: Unified lookup API across all scripts
- [ ] **`script_alphabet_values(script, system) -> Vec<(char, u32)>`**: Full mapping table per script
- [ ] **Cipher foundation**: Character↔number round-trip enables Caesar, Vigenère, substitution cipher implementations downstream (crypto crate or sankhya)

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
- [ ] ISO 639 validation (compile-time const table)
- [ ] Prosody patterns (intonation contours, rhythm class: stress/syllable/mora-timed)
- [ ] Morphological analyzer (stemming, lemmatization per language)
- [ ] Historical phonology (sound change rules, Proto-IE reconstructions)
- [ ] Sign language phonology (handshape, location, movement features)
- [ ] ScriptType::Featural for Hangul reclassification

## v1.0 Criteria

- [x] 50+ language inventories with verified phoneme data (51 languages)
- [x] All modules have 80%+ test coverage (98.53% measured)
- [x] Criterion benchmarks with 3-point trend history
- [x] Full serde roundtrip tests for all public types
- [ ] shabda + shabdakosh consuming varna for multilingual G2P (external crate work)
- [x] `[S]` sankhya consuming varna for script-aware numeral display and transliteration
- [x] Documentation: architecture overview, usage guide, API docs
- [x] English grammar profile added; 11 grammar profiles total
