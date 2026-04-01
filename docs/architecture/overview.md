# Architecture Overview

> **Lipi** — multilingual language engine

## Module Map

```
lipi/
├── src/
│   ├── lib.rs              — public API, module re-exports
│   ├── error.rs            — LipiError enum (non_exhaustive)
│   ├── phoneme/
│   │   ├── mod.rs          — IPA inventories, articulatory features, stress/tone,
│   │   │                     PhonemeInventoryBuilder, english/sanskrit/greek
│   │   ├── allophone.rs    — allophone rules, conditioned variants
│   │   ├── syllable.rs     — syllable structure, phonotactic constraints
│   │   └── inventories.rs  — 24 additional language inventories
│   ├── script/
│   │   ├── mod.rs          — writing system metadata, 10 scripts, by_code() lookup
│   │   ├── transliteration.rs — transliteration tables between scripts
│   │   └── numerals.rs     — numeral system mappings (5 systems)
│   ├── grammar/mod.rs      — 11 grammar profiles, morphology, word order, case systems
│   ├── lexicon/
│   │   ├── mod.rs          — vocabulary types, LexEntry, Lexicon, PartOfSpeech
│   │   ├── cognate.rs      — cross-language cognate tracking
│   │   └── swadesh.rs      — Swadesh-25 lists for 10 languages
│   ├── registry.rs         — 27 languages, central ISO 639 lookup
│   ├── dialect.rs          — language variety overlays (regional dialects, national standards)
│   ├── logging.rs          — optional LIPI_LOG env-based tracing init (feature: logging)
│   ├── mcp.rs              — 5 MCP tools (feature: mcp)
│   ├── daimon.rs           — agent registration (feature: daimon)
│   └── hoosh.rs            — LLM query interface (feature: hoosh)
├── benches/
│   └── benchmarks.rs       — criterion benchmarks
├── tests/
│   └── integration.rs      — cross-module integration tests
└── examples/
    └── basic.rs            — runnable usage example
```

## Data Flow

```
Language selection (ISO 639 code)
  │
  ├─→ registry  — central lookup: phonemes(), primary_script(), info()
  │     │
  │     ├─→ phoneme — IPA inventory, consonant/vowel counts, stress/tone pattern
  │     │     ├─→ allophone — conditioned variant rules
  │     │     └─→ syllable  — syllable structure, phonotactics
  │     └─→ script  — writing system type, direction, Unicode ranges
  │           ├─→ transliteration — inter-script romanization tables
  │           └─→ numerals        — script-specific numeral mappings
  │
  ├─→ grammar — morphology, word order, case/gender/number systems
  ├─→ lexicon — word lookup, Swadesh list, cognate relations
  └─→ dialect — variety overlays relative to parent language
```

## Registered Languages

| Code | Language          | Consonants | Vowels | Script      |
|------|-------------------|------------|--------|-------------|
| en   | English           | 24         | 12     | Latin       |
| sa   | Sanskrit          | 36         | 14     | Devanagari  |
| el   | Greek             | 20         | 5      | Greek       |
| ar   | Arabic            | 28         | 6      | Arabic      |
| hi   | Hindi             | 34         | 10     | Devanagari  |
| zh   | Mandarin Chinese  | 21         | 7      | CJK         |
| ja   | Japanese          | 20         | 5      | Kana        |
| es   | Spanish           | 23         | 5      | Latin       |
| fr   | French            | 21         | 16     | Latin       |
| de   | German            | 23         | 16     | Latin       |
| ru   | Russian           | 36         | 6      | Cyrillic    |
| ko   | Korean            | 19         | 7      | Hangul      |
| pt   | Portuguese        | 23         | 14     | Latin       |
| la   | Latin             | 18         | 10     | Latin       |
| grc  | Koine Greek       | 17         | 5      | Greek       |
| lzh  | Literary Chinese  | 27         | 12     | CJK         |
| sw   | Swahili           | 26         | 5      | Latin       |
| tr   | Turkish           | 20         | 8      | Latin       |
| vi   | Vietnamese        | 22         | 11     | Latin       |
| fi   | Finnish           | 17         | 16     | Latin       |
| th   | Thai              | 21         | 9      | Thai        |
| yo   | Yoruba            | 18         | 7      | Latin       |
| zu   | Zulu              | 42         | 5      | Latin       |
| tl   | Tagalog           | 18         | 5      | Latin       |
| haw  | Hawaiian          | 8          | 10     | Latin       |
| nah  | Nahuatl           | 16         | 8      | Latin       |
| yua  | Yucatec Maya      | 21         | 10     | Latin       |

## Registered Scripts

| Code | Name                  | Type        | Direction | Status     |
|------|-----------------------|-------------|-----------|------------|
| Latn | Latin                 | Alphabet    | LTR       | Living     |
| Arab | Arabic                | Abjad       | RTL       | Living     |
| Deva | Devanagari            | Abugida     | LTR       | Living     |
| Hani | CJK Unified Ideographs | Logographic | LTR      | Living     |
| Cyrl | Cyrillic              | Alphabet    | LTR       | Living     |
| Hang | Hangul                | Alphabet    | LTR       | Living     |
| Kana | Kana (Hiragana + Katakana) | Syllabary | LTR    | Living     |
| Grek | Greek                 | Alphabet    | LTR       | Living     |
| Xsux | Cuneiform             | Logographic | LTR       | Historical |
| Egyp | Egyptian Hieroglyphs  | Logographic | RTL       | Historical |

## Numeral Systems

| Script | System Name           | Kind        |
|--------|-----------------------|-------------|
| Grek   | Greek Isopsephy       | Alphabetic  |
| Deva   | Devanagari Digits     | Decimal     |
| Arab   | Arabic-Indic Digits   | Decimal     |
| Xsux   | Babylonian Sexagesimal | Other      |
| Egyp   | Egyptian Fractions    | Other       |

## Dependency Stack

```
lipi (this crate)
  │
  ├── serde          — serialization for all types (alloc; std optional)
  ├── thiserror      — error derivation (std optional)
  ├── tracing        — structured logging
  │
  └── optional (feature-gated):
      ├── serde_json       — JSON encoding (mcp, daimon, hoosh)
      └── tracing-subscriber — env-filter log init (logging)
```

## Feature Flags

| Flag    | Enables                                    |
|---------|--------------------------------------------|
| std     | std-backed serde + thiserror (default)     |
| logging | tracing-subscriber, LIPI_LOG env init      |
| mcp     | 5 MCP tool definitions (requires std)      |
| daimon  | agent registration interface (requires std)|
| hoosh   | LLM query interface (requires std)         |
| full    | all of the above                           |

## Downstream Consumers

```
lipi
  ├─→ shabda      — G2P conversion (multilingual phoneme sets)
  ├─→ shabdakosh  — pronunciation dictionary (IPA dictionaries)
  ├─→ svara       — vocal synthesis (phoneme-to-audio)
  ├─→ sankhya     — ancient mathematical systems (script-aware numerals, Babylonian/Egyptian)
  ├─→ jnana       — multilingual knowledge access
  ├─→ vidya       — programming reference (native language explanations)
  ├─→ vansh       — voice assistant TTS/STT (planned)
  └─→ sahifa      — OCR language detection, multilingual document processing (planned)
```

## Design Principles

- **Data-driven**: Language data as structured Rust types, not embedded strings
- **Queryable**: Every inventory supports lookup, filtering, counting, and by_code() dispatch
- **Composable**: Each module is independent — consumers pull only what they need
- **Serializable**: All types implement Serialize + Deserialize for data exchange
- **Extensible**: `#[non_exhaustive]` on all enums — new variants without breaking changes
- **Zero-alloc statics**: `Cow<'static, str>` for all pre-built inventory data
- **Builder pattern**: `PhonemeInventoryBuilder` for ergonomic inventory construction
- **Feature-gated optionals**: logging, mcp, daimon, and hoosh add zero cost when unused
- **Tracing throughout**: every lookup emits a `trace!` span for audit and debugging
