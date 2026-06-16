# ADR-003: Builder Pattern for Phoneme Inventories

> **Historical — pre-2.0 Rust crate.** The builder pattern survives the Cyrius port; the mechanics changed: chained methods become builder free functions, `#[must_use]` → the `#must_use` directive, and debug-mode duplicate detection → `lib/assert.cyr` under a debug define. The Rust code block below is illustrative — see the Cyrius form in [usage.md](../guides/usage.md) and [ADR 0001](0001-port-from-rust-to-cyrius.md).

## Status

Accepted

## Context

Phoneme inventories contain 5-50+ phonemes, each with IPA symbol, manner, place, voicing (consonants) or height, backness, rounding (vowels), plus optional tones and stress pattern. Construction options:

1. **Struct literal** — verbose, error-prone for large inventories
2. **Builder pattern** — chained method calls, type-safe, readable
3. **Data-driven (JSON/TOML)** — load from files at runtime
4. **Macro** — DSL for inventory construction

## Decision

Use a `PhonemeInventoryBuilder` with chained `.consonant()` and `.vowel()` methods. Pre-built inventories are functions that return `PhonemeInventory` via the builder.

```rust
PhonemeInventoryBuilder::with_capacity("en", "English", 36)
    .stress(StressPattern::Free)
    .consonant("p", Plosive, Bilabial, false)
    .vowel("iː", Close, Front, false)
    .build()
```

## Consequences

**Benefits:**
- Self-documenting: each phoneme's features are explicit at the call site
- `#[must_use]` on every builder method prevents silent drops
- `with_capacity` hint avoids reallocation for known inventory sizes
- Debug-mode duplicate detection catches IPA symbol collisions
- Consumers can build custom inventories using the same API

**Trade-offs:**
- Each pre-built inventory is a function call, not a static const — small runtime cost on first access
- No compile-time validation of phonological plausibility (e.g., a "voiced bilabial fricative" is valid syntax even if linguistically unusual)
- Future data-driven loading (PHOIBLE CSV) will need a separate path alongside the builder

**Validation:**
- All 51 language inventories use the builder without issues
- Benchmark: English inventory builds in ~140ns, Sanskrit in ~196ns
