# Security Policy

## Scope

Varna is a pure language-data library (Cyrius) providing phoneme inventories, writing
system metadata, grammar profiles, and lexicon access. The core engine performs no I/O
and uses no `@unsafe` blocks.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| String processing | Unicode edge cases, normalization | `lib/str.cyr` + `lib/unicode` handling; no raw pointer arithmetic in the data path |
| JSON ingestion | Crafted JSON | `bayan` (`lib/bayan.cyr`) `json_parse`/`json_get` with explicit field extraction + tagged-enum validation |
| Phoneme lookup | Linear scan on large inventories | Bounded by inventory size (~50 phonemes per language) |
| Lexicon search | Large word lists | Consumer responsibility for input bounds |
| AI surfaces (opt-in) | JSON output for daimon/hoosh; MCP via bote-core | Compiled only under `-D DAIMON`/`-D HOOSH`/`-D MCP`; absent by default |
| Dependencies | Supply chain compromise | `cyrius vet` / `cyrius deny`; 0 external (non-Cyrius) deps — stdlib + the `bote-core` git dep only |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 2.x | Yes (Cyrius) |
| 1.x | No (Rust crate, frozen in `rust-old/`) |
| < 1.0 | No |

## Reporting

- Contact: **security@agnos.dev**
- Do not open public issues for security vulnerabilities
- 48-hour acknowledgement SLA
- 90-day coordinated disclosure

## Design Principles

- No `@unsafe` blocks
- No implicit aborts — every fallible call returns a sentinel/`Result`, checked by the caller
- No network I/O in the core engine (AI surfaces are opt-in via `-D` defines)
- Minimal dependency surface — Cyrius stdlib (`lib/str.cyr`, `lib/bayan.cyr`, …) plus the
  `bote-core` git dep under `-D MCP`; zero external (non-Cyrius) dependencies
