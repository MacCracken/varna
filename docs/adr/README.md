# Architecture Decision Records

Decisions about varna — what we chose, the context, and the consequences we accept. Use these when a future reader would reasonably ask *"why did we do it this way?"*

## Conventions

- **Filename**: `NNNN-kebab-case-title.md`, zero-padded to four digits. Never renumber.
- **One decision per ADR.** If a decision supersedes a prior one, add a new ADR and set the old one's status to `Superseded by NNNN`.
- **Status lifecycle**: `Proposed` → `Accepted` → (optionally) `Superseded` or `Deprecated`.
- Use [`template.md`](template.md) as the starting point.

## ADR vs. architecture note vs. guide

| Kind | Lives in | Answers |
|---|---|---|
| ADR | `docs/adr/` | *Why did we choose X over Y?* |
| Architecture note | `docs/architecture/` | *What non-obvious constraint is true about the code?* |
| Guide | `docs/guides/` | *How do I do X?* |

## Index

- [0001 — Port from Rust to Cyrius](0001-port-from-rust-to-cyrius.md) — Accepted (2026-06-16); its `rust-old/` freeze superseded by 0002
- [0002 — Remove the `rust-old/` archive](0002-remove-the-rust-old-archive.md) — Accepted (2026-08-27)

Historical (pre-2.0 Rust crate; 3-digit numbering; type/build decisions superseded by 0001):

- [001 — Cow<'static, str> over String](001-cow-over-string.md)
- [002 — Flat Module Architecture](002-flat-module-architecture.md)
- [003 — Builder Pattern for Phoneme Inventories](003-builder-pattern-for-inventories.md)
- [004 — Forward-Map Transliteration Tables](004-transliteration-table-design.md)
- [005 — Feature-Gated AI Integration Modules](005-feature-gated-ai-modules.md)
