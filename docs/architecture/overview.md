# Architecture Overview

> **Varna** — multilingual language engine (Cyrius)

## Module Map

```
varna/
├── src/
│   ├── main.cyr           — demo entry + include aggregator (alloc_init, dispatch)
│   ├── error.cyr          — VarnaError enum (tagged variants)
│   ├── phoneme.cyr        — IPA inventories, articulatory features, stress/tone,
│   │                        the inventory builder, english/sanskrit/greek
│   ├── inventories.cyr    — extended language inventories
│   ├── allophone.cyr      — allophone rules, conditioned variants
│   ├── syllable.cyr       — syllable structure, phonotactic constraints
│   ├── script.cyr         — writing system metadata, scripts, by_code() lookup
│   ├── transliteration.cyr — transliteration tables between scripts
│   ├── numerals.cyr       — numeral system mappings (Greek/Deva/Babylonian/Egyptian/rod)
│   ├── grammar.cyr        — grammar profiles, morphology, word order, case systems
│   ├── lexicon.cyr        — vocabulary types, LexEntry, Lexicon, PartOfSpeech
│   ├── swadesh.cyr        — Swadesh-25 lists per language
│   ├── cognate.cyr        — cross-language cognate tracking
│   ├── dialect.cyr        — language-variety overlays (regional dialects, national standards)
│   ├── registry.cyr       — central ISO 639 lookup across all domains
│   ├── logging.cyr        — VARNA_LOG env init (log + sakshi)        [-D LOGGING]
│   ├── mcp.cyr            — 5 MCP tools, bote-core surface           [-D MCP]
│   ├── daimon.cyr        — agent registration payload                [-D DAIMON]
│   └── hoosh.cyr         — LLM query interface                       [-D HOOSH]
├── lib/                   — vendored Cyrius stdlib snapshot (cyrius deps; gitignored)
├── dist/
│   └── varna.cyr          — single-file bundle for consumers (cyrius distlib)
└── tests/
    ├── *.tcyr             — cross-module integration tests (cyrius tests)
    └── *.bcyr             — cyrius bench harness (lib/bench.cyr)
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

51 languages are registered in total (the table above lists the core set; see
`registry.cyr` and `inventories.cyr` for the full roster).

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
varna (this engine)
  │  Cyrius stdlib (resolved by `cyrius deps` into lib/)
  ├── bayan           — JSON serialization for all types (#derive(Serialize), json_parse/json_build)
  ├── result + tagged — native enum + Result<T,E> error model (VarnaError)
  ├── str / string / slice / vec / hashmap / fmt / io — core data structures + formatting
  │
  └── optional (define-gated):
      ├── log + sakshi               — structured logging + audit         (-D LOGGING)
      └── bote (dist/bote-core.cyr)  — MCP / JSON-RPC core surface        (-D MCP)
```

The v1.x crate's external dependencies all resolve to Cyrius stdlib or builtins:

| Rust dependency | Cyrius equivalent | Kind |
|-----------------|-------------------|------|
| `serde` + `serde_json` | `bayan` — `#derive(Serialize)` + `json_parse`/`json_build` | stdlib |
| `thiserror` | native `enum` + `Result<T,E>` (`lib/result.cyr`, `lib/tagged.cyr`) | builtin |
| `tracing` | `lib/log.cyr` (level filtering) | stdlib |
| `tracing-subscriber` | `lib/sakshi.cyr` (output routing; env-filter wired by hand) | stdlib |
| `criterion` | `cyrius bench` + `lib/bench.cyr` | toolchain |

Deserialization is hand-rolled via `bayan` `json_parse`/`json_get` accessors — Cyrius
ships `#derive(Serialize)` but no `#derive(Deserialize)` counterpart.

## Build Defines

| Define | Enables |
|--------|---------|
| _(none)_ | core data engine (phoneme/script/grammar/lexicon/registry/dialect) |
| `-D LOGGING` | `log`+`sakshi` init, `VARNA_LOG` env var |
| `-D MCP` | 5 MCP tool definitions (folds `bote-core`) |
| `-D DAIMON` | agent registration payload |
| `-D HOOSH` | LLM query interface |

Pass any combination on the `cyrius build` line; passing all four is the `full` equivalent.

## Downstream Consumers

```
varna
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

- **Data-driven**: Language data as structured Cyrius `struct`s, not embedded strings
- **Queryable**: Every inventory supports lookup, filtering, counting, and `by_code()` dispatch
- **Composable**: Each module is independent — consumers fold only the bundle they need (`dist/varna-core.cyr` vs `dist/varna.cyr`)
- **Serializable**: All types emit JSON via `#derive(Serialize)` / `bayan` for data exchange
- **Extensible**: tagged enums everywhere — new variants are additive, no break for consumers
- **Static-literal data**: pre-built inventories hold `'static` `str` literals — no per-build heap churn (Cyrius has no `Cow`)
- **Builder pattern**: an inventory builder for ergonomic construction
- **Define-gated optionals**: `-D LOGGING`/`MCP`/`DAIMON`/`HOOSH` add zero cost when not passed
- **Logging throughout**: every lookup emits a `log`/`sakshi` event for audit and debugging (under `-D LOGGING`)
