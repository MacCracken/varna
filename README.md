# Varna

> **Varna** (Sanskrit: वर्ण — letter, character, sound) — multilingual language engine for AGNOS

Structured, queryable corpus of human language data. Phoneme inventories, writing
system metadata, grammar profiles, and lexicon access for 50+ languages — written in
[Cyrius](https://github.com/MacCracken/cyrius). (Migrated from a Rust crate at v2.0;
the frozen Rust source lives in `rust-old/` — see [ADR 0001](docs/adr/0001-port-from-rust-to-cyrius.md).)

Used by [shabda](https://github.com/MacCracken/shabda) (G2P conversion),
[shabdakosh](https://github.com/MacCracken/shabdakosh) (pronunciation dictionary),
[svara](https://github.com/MacCracken/svara) (vocal synthesis),
[sankhya](https://github.com/MacCracken/sankhya) (ancient mathematical systems),
[jnana](https://github.com/MacCracken/jnana) (knowledge system), and
[vidya](https://github.com/MacCracken/vidya) (programming reference).

## Modules

| Module | Description |
|--------|-------------|
| `phoneme` | IPA phoneme inventories per language, articulatory features (manner, place, voicing), stress/tone patterns |
| `script` | Writing system metadata: alphabet, syllabary, logographic, abjad, abugida. Unicode ranges, directionality |
| `grammar` | Morphological typology (isolating, agglutinative, fusional, polysynthetic), word order, case systems |
| `lexicon` | Core vocabulary per language (Swadesh lists, frequency-ranked word lists), cognate detection |
| `registry` | Central ISO 639 lookup across phoneme/script/grammar/lexicon |
| `dialect` | Language-variety overlays (regional dialects, national standards) |

## Build Defines

Optional integration surfaces compile in only when their `-D` define is passed to
`cyrius build`. The core data engine (phoneme/script/grammar/lexicon/registry/dialect)
is always present.

| Define | Enables |
|--------|---------|
| _(none)_ | Core linguistic data engine |
| `-D LOGGING` | Structured logging via the `VARNA_LOG` env var (`lib/log.cyr` + `lib/sakshi.cyr`) |
| `-D MCP` | 5 MCP tool definitions for AI-agent integration (folds `bote-core`) |
| `-D DAIMON` | AGNOS agent-framework registration payload |
| `-D HOOSH` | Structured LLM query types + `answer_from_data()` |

`cyrius build -D LOGGING -D MCP -D DAIMON -D HOOSH src/main.cyr build/varna` enables everything.

## Quick Start

Depend on varna from another Cyrius project by adding it to `cyrius.cyml`:

```cyml
[deps.varna]
git = "https://github.com/MacCracken/varna"
tag = "2.0.0"
modules = ["dist/varna.cyr"]
```

`cyrius deps` clones varna at the tag and copies the bundle into `lib/varna.cyr`:

```cyrius
include "lib/varna.cyr"

fn main() {
    alloc_init();

    # Get the English (General American) phoneme inventory
    var en = phoneme_english();
    var has_th = phoneme_has(en, "θ");      # voiceless dental fricative (think)
    var has_dh = phoneme_has(en, "ð");      # voiced dental fricative (this)
    var no_r   = phoneme_has(en, "ʀ");      # 0 — no uvular trill in English

    var consonants = phoneme_consonant_count(en);
    var vowels     = phoneme_vowel_count(en);

    # Look up a specific phoneme — returns a sentinel (0) when absent
    var sh = phoneme_find(en, "ʃ");         # postalveolar fricative (ship)

    return 0;
}
```

The Cyrius API mirrors the v1.x Rust surface as snake-case free functions
(`varna::phoneme::english()` → `phoneme_english()`, `inv.has("θ")` → `phoneme_has(inv, "θ")`),
with sentinel-return checks in place of `Option`/`unwrap`.

## Dependency Stack

```
varna (this engine)
  │  Cyrius stdlib (resolved by `cyrius deps` into lib/)
  ├── bayan          — JSON serialization (#derive(Serialize), json_parse/json_build)
  ├── result + tagged — native enum + Result<T,E> error model (VarnaError)
  ├── str / string / slice / vec / hashmap / fmt / io — core data structures + formatting
  ├── log + sakshi   — structured logging + audit (only under -D LOGGING)
  │
  └── git dep (only under -D MCP):
      └── bote (dist/bote-core.cyr) — MCP / JSON-RPC core surface
```

Zero external (non-Cyrius) dependencies. The v1.x crate's `serde`, `serde_json`,
`thiserror`, `tracing`, `tracing-subscriber`, and `criterion` all resolve to Cyrius
stdlib modules or language builtins — see [ADR 0001](docs/adr/0001-port-from-rust-to-cyrius.md).

## Architecture

```text
varna (this) — language structure & phoneme inventories
  | provides phoneme sets per language
shabda — G2P conversion (currently English-only, varna makes it multilingual)
  | produces phoneme sequences
shabdakosh — pronunciation dictionary (currently CMUdict, varna adds IPA dicts)
  | lookup fallback
svara — vocal synthesis (consumes phonemes, produces audio)
  | voice output
dhvani — audio engine (mixing, DSP, output)
```

Also feeds:
- **sankhya** — ancient mathematical systems (script-aware numerals, Babylonian/Egyptian)
- **jnana** — multilingual knowledge access
- **vidya** — programming concepts explained in native languages
- **vansh** (planned) — voice assistant with multilingual TTS/STT
- **sahifa** (planned) — OCR language detection, multilingual document processing

## Development

```bash
cyrius deps                          # resolve stdlib + bote into lib/
cyrius build src/main.cyr build/varna # compile the engine + demo entry
cyrius distlib                       # bundle src/ → dist/varna.cyr for consumers

sh scripts/check.sh                  # local gate: deps + fmt + lint + build + tests
cyrius tests                         # run tests/*.tcyr (recursive)
cyrius bench tests/varna.bcyr        # run benchmarks (tests/*.bcyr)
./scripts/bench-history.sh           # benchmarks with CSV trend history
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full toolchain reference.

## License

GPL-3.0-only. See [LICENSE](LICENSE).
